use ketchup::error::KError;
use logos::SpannedIter;
use crate::lang::{error::parser::Error, parser::{expr::parse_expr, stmt::parse_stmt}, token::Token, Spanned};
use super::stmt::Stmt;

/// Parses an if-(else)? statement (given that the `if` token has already been consumed)
pub fn parse_if(tokens: &mut SpannedIter<'_, Token>) -> Result<(Spanned<Stmt>, Option<Spanned<Result<Token, Error>>>), Vec<KError<Error>>> {
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
        _ => return Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses { ctx_span: lparen_span })])
    }

    // get the body statement of the if statement
    let (body, next) = parse_stmt(tokens.next(), tokens)?;

    // check for else keyword
    if let Some((Ok(Token::Else), _)) = next {
        // get the else body
        let (otherwise, next) = parse_stmt(tokens.next(), tokens)?;

        // return the completed otherwise statement
        let span = start_span.start..otherwise.1.end;
        return Ok((
            (
                Stmt::If { cond, body: Box::new(body), otherwise: Some(Box::new(otherwise)) },
                span,
            ),
            next,
        ));
    }

    // return the completed if statement
    let span = start_span.end..body.1.end;
    Ok((
        (
            Stmt::If { cond, body: Box::new(body), otherwise: None },
            span,
        ),
        next,
    ))
}
