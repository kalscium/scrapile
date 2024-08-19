use ketchup::{error::KError, Span};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};
use super::expr::{parse_expr, Expr};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
}

pub fn parse_stmt(first_tok: Option<(Result<Token, Error>, Span)>, tokens: &mut SpannedIter<'_, Token>) -> Result<((Stmt, Span), Option<(Result<Token, Error>, Span)>), Vec<KError<Token, Error>>> {
    let (first_tok, start_span) = match first_tok {
        Some((Ok(tok), span)) => (tok, span),
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
        None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedStmt)]),
    };

    match first_tok {
        // assume that the statement is a expr
        _ => parse_expr(Some((Ok(first_tok), start_span.clone())), tokens).map(|(expr, next_tok)| ((Stmt::Expr(expr.clone()), start_span.start..expr.span.end), next_tok.map(|(tok, span)| (Ok(tok), span)))),
    }
}
