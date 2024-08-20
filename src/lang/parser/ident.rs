use ketchup::error::KError;
use logos::SpannedIter;
use crate::lang::{error::parser::Error, token::Token, Spanned};
use super::{expr::{Expr, ExprOper}, tuple::parse_tuple};

#[derive(Debug, Clone)]
pub struct Call {
    pub ident: Vec<String>,
    pub args: Vec<Expr>,
    pub is_builtin: bool,
}

pub fn parse_call_or_ident(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<(Spanned<ExprOper>, Option<Spanned<Result<Token, Error>>>), Vec<KError<Error>>> {
    let ((ident, start_span), next_tok) = parse_ident(ident, tokens)?;

    // check for function call
    if let Some((token, span)) = next_tok {
        if token == Token::LParen {
            // if there is a function call then parse the arguments of it
            let (args, span) = parse_tuple(tokens)?;
            return Ok(((ExprOper::Call( Call {
                ident,
                args,
                is_builtin: false,
            }), start_span.start..span.end), tokens.next()));
        } else {
            // if the token found is not of a function call (`LParen`)
            return Ok(((ExprOper::Ident(ident), start_span), Some((Ok(token), span))));
        }
    }

    // if there are no tokens following the identifier
    Ok(((ExprOper::Ident(ident), start_span), None))
}

pub fn parse_call(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<Spanned<Call>, Vec<KError<Error>>> {
    let ((ident, start_span), next_tok) = parse_ident(ident, tokens)?;

    // check for function call
    if let Some((token, _)) = next_tok {
        if token == Token::LParen {
            // if there is a function call then parse the arguments of it
            let (args, span) = parse_tuple(tokens)?;
            return Ok((Call {
                ident,
                args,
                is_builtin: false,
            }, start_span.start..span.end));
        }
    }

    // if there are no tokens following the identifier or there isn't a LParen following the ident
    Err(vec![KError::Other(tokens.span(), Error::ExpectedCallLParen { ctx_span: start_span })])
}

pub fn parse_ident(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<(Spanned<Vec<String>>, Option<Spanned<Token>>), Vec<KError<Error>>> {
    let start_span = tokens.span();

    let mut idents = vec![ident];

    // check for sub-identifiers
    let mut past_span = start_span.clone();
    while let Some((token, span)) = tokens.next() {
        let token = token.map_err(|err| vec![KError::Other(span.clone(), err)])?;
        if token != Token::Dot { // if the identifier is not continued by a dot
            return Ok(((idents, start_span.start..past_span.end), Some((token, span))));
        }

        past_span = tokens.span();

        // parse for the ident part of the sub-identifier
        idents.push(match tokens.next() {
            Some((Ok(Token::Ident(ident)), _)) => ident,
            Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
            _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedIdent)]),
        });
    }

    // if there are no more tokens left after the identifier
    Ok(((idents, start_span.start..tokens.span().end), None))
}
