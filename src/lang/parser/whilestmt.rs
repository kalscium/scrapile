use ketchup::error::KError;
use logos::SpannedIter;
use crate::lang::{error::parser::Error, parser::{expr::parse_expr, stmt::parse_stmt}, token::Token, Spanned};
use super::stmt::Stmt;

/// Parses a while statement (given that the `while` token has already been consumed)
pub fn parse_while(tokens: &mut SpannedIter<'_, Token>) -> Result<(Spanned<Stmt>, Option<Spanned<Result<Token, Error>>>), Vec<KError<Error>>> {
    let start_span = tokens.span();

    // make sure the condition is wrapped in parentheses
    let lparen_span = match tokens.next() {
        Some((Ok(Token::LParen), span)) => span,
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),

        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedCondLParen { ctx_span: start_span })]),
    };


    // get the condition expression of the if statement
    let (cond, next) = parse_expr(tokens.next(), tokens)?;

    // make sure the parentheses are terminated
    match next {
        Some((Token::RParen, _)) => (),
        _ => return Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses { ctx_span: lparen_span })]),
    }

    // get the body statement of the while statement
    let (body, next) = parse_stmt(tokens.next(), tokens)?;

    // return the completed while statement
    let span = start_span.start..body.1.end;
    Ok((
        (
            Stmt::While { cond, body: Box::new(body) },
            span,
        ),
        next,
    ))
}
