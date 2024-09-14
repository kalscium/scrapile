use ketchup::{error::KError, Span};
use logos::SpannedIter;
use crate::lang::{error::parser::Error, parser::{expr::parse_expr, types::parse_type}, token::Token, Spanned};
use super::stmt::Stmt;

/// Parses variable delcaration / definition (given that the `Let` token as already been consumed)
pub fn parse_var_declare(tokens: &mut SpannedIter<'_, Token>) -> Result<(Spanned<Stmt>, Option<(Result<Token, Error>, Span)>), Vec<KError<Error>>> {
    let start_span = tokens.span();

    // variables that get mutated later
    let mut mutable = false;
    let mut atype = None;

    // get the first token
    let mut token = match tokens.next() {
        Some((Ok(Token::Ident(ident)), _)) => Token::Ident(ident),
        Some((Ok(Token::Mut), _)) => Token::Mut,

        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedMutOrIdent { ctx_span: start_span.start..tokens.span().end })]),
    };

    // check for the mutability token
    if token == Token::Mut {
        mutable = true;

        // update the next token to be the identifier
        token = match tokens.next() {
            Some((Ok(token), _)) => token,
            Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
            None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedIdent { ctx_span: start_span.start..tokens.span().end })]),
        };
    }

    // get the identifier token
    let ident = match token {
        Token::Ident(ident) => ident,
        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedIdent { ctx_span: start_span.start..tokens.span().end })]),
    };

    // get next token
    let token = match tokens.next() {
        Some((Ok(Token::Colon), _)) => Token::Colon,
        Some((Ok(Token::EQ), _)) => Token::EQ,

        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedColonOrEQ { ctx_span: start_span.start..tokens.span().end })]),
    };

    // check for type annotations
    if token == Token::Colon {
        atype = Some(parse_type(tokens.next(), tokens)?);

        // check for `eq`
        match tokens.next() {
            Some((Ok(Token::EQ), _)) => token,

            Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
            _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedEQ { ctx_span: start_span.start..tokens.span().end })]),
        };
    }

    // `eq` already checked for by the previous two blocks

    // get the value of the variable
    let (value, next_tok) = parse_expr(tokens.next(), tokens)?;
    let span = start_span.start..value.span.end;
    
    // return completed variable declaration statement
    Ok(((Stmt::VarDeclare {
        mutable,
        ident,
        atype,
        value,
    }, span), next_tok.map(|(tok, span)| (Ok(tok), span))))
}
