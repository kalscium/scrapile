use ketchup::{error::KError, Span};
use logos::SpannedIter;
use crate::lang::{error::parser::Error, token::Token, typed::types::Type, Spanned};
use super::{expr::{parse_expr, Expr}, variables::{parse_var_declare, parse_var_mutate}};

#[derive(Debug, Clone)]
pub enum Stmt {
    /// An expression
    Expr(Expr),

    /// A variable declaration with `let`
    VarDeclare {
        mutable: bool,
        ident: String,
        atype: Option<Spanned<Type>>,
        value: Expr,
    },

    /// A variable mutation
    VarMutate {
        ident: (String, Span),
        value: Expr,
    },
}

pub fn parse_stmt(first_tok: Option<Spanned<Result<Token, Error>>>, tokens: &mut SpannedIter<'_, Token>) -> Result<(Spanned<Stmt>, Option<Spanned<Result<Token, Error>>>), Vec<KError<Error>>> {
    let (first_tok, start_span) = match first_tok {
        Some((Ok(tok), span)) => (tok, span),
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
        None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedStmt)]),
    };

    match first_tok {
        Token::Let => parse_var_declare(tokens),
        Token::Mut => parse_var_mutate(tokens),
        
        // assume that the statement is a expr
        _ => parse_expr(Some((Ok(first_tok), start_span.clone())), tokens).map(|(expr, next_tok)| ((Stmt::Expr(expr.clone()), start_span.start..expr.span.end), next_tok.map(|(tok, span)| (Ok(tok), span)))),
    }
}
