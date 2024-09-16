use ketchup::error::KError;
use logos::SpannedIter;
use crate::lang::{error::parser::Error, token::Token, typed::types::Type, Spanned};

/// Parses a type usage
pub fn parse_type(first_tok: Option<Spanned<Result<Token, Error>>>, tokens: &mut SpannedIter<'_, Token>) -> Result<Spanned<Type>, Vec<KError<Error>>> {
    // get the first token for the type annotation
    let (token, start_span) = match first_tok {
        Some((token, span)) => (token.map_err(|err| vec![KError::Other(span.clone(), err)])?, span),
        None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedType)]),
    };

    // match the token
    match token {
        // named types
        Token::Ident(ident) => Ok((match &*ident {
            // primatives
            "str" => Type::String,
            "num" => Type::Number,
            "bool" => Type::Bool,
            "nil" => Type::Nil,
            
            // custom uesr-defined types
            _ => Type::Custom { ident },
        }, start_span)),

        // tuples
        Token::LParen => Ok((Type::Tuple(parse_tuple_type(tokens)?), start_span.start..tokens.span().end)),

        // lists
        Token::LBracket => Ok((parse_list_type(tokens)?, start_span.start..tokens.span().end)),

        // invalid types
        _ => Err(vec![KError::Other(start_span, Error::ExpectedType)]),
    }
}

fn parse_tuple_type(tokens: &mut SpannedIter<'_, Token>) -> Result<Vec<Type>, Vec<KError<Error>>> {
    let start_span = tokens.span();
    let mut types = Vec::new();

    // get the first token of the tuple type and also check for `)` for nill tuples
    let first_tok = match tokens.next() {
        Some((Ok(token), span)) => {
            if token == Token::RParen {
                return Ok(types); // return empty tuple type
            }

            // return token
            Some((Ok(token), span))
        },
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
        None => return Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses { ctx_span: start_span })]), // if there is never a `)`
    };

    // parse the first type
    let (atype, _) = parse_type(first_tok, tokens)?;
    types.push(atype);

    while let Some((token, span)) = tokens.next() {
        let token = token.map_err(|err| vec![KError::Other(span.clone(), err)])?;
        match token {
            Token::RParen => return Ok(types), // when the tuple type is terminated
            Token::Comma => {
                match tokens.next() {
                    Some((Ok(Token::RParen), _)) => return Ok(types),
                    Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
                    None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedCommaOrRParen { ctx_span: start_span })]),

                    // parse next type
                    token => types.push(parse_type(token, tokens)?.0),
                }
            },
            _ => return Err(vec![KError::Other(span, Error::ExpectedCommaOrRParen { ctx_span: start_span })]),
        }
    }

    // this section of code can only be reached when the tuple is never terminated with `)`
    Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses { ctx_span: start_span })])
}

fn parse_list_type(tokens: &mut SpannedIter<'_, Token>) -> Result<Type, Vec<KError<Error>>> {
    let start_span = tokens.span();

    // get the type of the list
    let (list_type, _) = parse_type(tokens.next(), tokens)?;

    // make sure the list is terminated
    match tokens.next() {
        Some((Ok(Token::RBracket), _)) => (),
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
        _ => return Err(vec![KError::Other(tokens.span(), Error::UnclosedBrackets { ctx_span: start_span })]), // if there isn't a `]` found
    };

    // return list type
    Ok(Type::List(Box::new(list_type)))
}
