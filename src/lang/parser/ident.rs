use ketchup::{error::KError, Span};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};
use super::{expr::ExprOper, tuple::parse_tuple};

pub fn parse_call_or_ident(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<((ExprOper, Span), Option<(Result<Token, Error>, Span)>), Vec<KError<Token, Error>>> {
    let ((ident, start_span), next_tok) = parse_ident(ident, tokens)?;

    // check for function call
    if let Some((token, span)) = next_tok {
        if token == Token::LParen {
            // if there is a function call then parse the arguments of it
            let (args, span) = parse_tuple(tokens)?;
            return Ok(((ExprOper::Call {
                ident,
                args,
            }, start_span.start..span.end), tokens.next()));
        } else {
            // if the token found is not of a function call (`LParen`)
            return Ok(((ExprOper::Ident(ident), start_span), Some((Ok(token), span))));
        }
    }

    // if there are no tokens following the identifier
    Ok(((ExprOper::Ident(ident), start_span), None))
}

pub fn parse_ident(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<((Vec<String>, Span), Option<(Token, Span)>), Vec<KError<Token, Error>>> {
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
