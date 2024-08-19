use ketchup::{error::KError, Span};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};
use super::expr::{parse_expr, Expr};

/// Parses a tuple (given that the `LParen` token has already been consumed)
pub fn parse_tuple(tokens: &mut SpannedIter<'_, Token>) -> Result<(Vec<Expr>, Span), Vec<KError<Error>>> {
    let start_span = tokens.span();
    let mut exprs = Vec::new();

    // get the first token of the tuple and also check for `)` for empty tuples
    let first_tok = match tokens.next() {
        Some((token, span)) => {
            let token = token.map_err(|err| vec![KError::Other(span.clone(), err)])?;
            if token == Token::RParen {
                return Ok((exprs, start_span.start..span.end)); // return empty tuple
            }

            // return token
            Some((Ok(token), span))
        },
        None => return Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses)]), // if there is never a `)`
    };

    // parse first expr
    let (expr, mut next_tok) = parse_expr(first_tok, tokens)?;
    exprs.push(expr);

    while let Some((token, span)) = next_tok {
        match token {
            Token::RParen => return Ok((exprs, start_span.start..span.end)), // when the tuple is terminated
            Token::Comma => {
                match tokens.next() {
                    Some((Ok(Token::RParen), span)) => return Ok((exprs, start_span.start..span.end)),
                    Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
                    None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedCommaOrRParen)]),

                    token => {
                        let (expr, local_next_tok) = parse_expr(token, tokens)?; // parse next expr
                        next_tok = local_next_tok; // update `next_tok` to be the token after the expr
                        exprs.push(expr); // add the expr to the list of exprs
                    },
                }
            },
            _ => return Err(vec![KError::Other(span, Error::ExpectedCommaOrRParen)]),
        }
    }

    // this section of code can only be reached when the tuple is never terminated with `)`
    Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses)])
}
