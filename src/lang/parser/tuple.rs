use ketchup::{error::KError, node::Node, Span};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};
use super::expr::{parse_expr, ExprOper};

/// Parses a tuple (given that the `LParen` token has already been consumed)
pub fn parse_tuple(tokens: &mut SpannedIter<'_, Token>) -> Result<(Vec<Vec<Node<ExprOper>>>, Span), Vec<KError<Token, Error>>> {
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
    let ((expr, _), mut next_tok) = parse_expr(first_tok, tokens)?;
    exprs.push(expr);

    while let Some((token, span)) = next_tok {
        match token {
            Token::RParen => return Ok((exprs, start_span.start..span.end)), // when the tuple is terminated
            Token::Comma => {
                let ((expr, _), local_next_tok) = parse_expr(tokens.next(), tokens)?; // parse next expr
                next_tok = local_next_tok; // update `next_tok` to be the token after the expr
                exprs.push(expr); // add the expr to the list of exprs

                continue
            },
            _ => return Err(vec![KError::Other(span, Error::ExpectedCommaOrRParen)]),
        }
    }

    // this section of code can only be reached when the tuple is never terminated with `)`
    Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses)])
}
