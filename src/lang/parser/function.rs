//! Parsing for function definitions

use ketchup::{error::KError, Span};
use logos::SpannedIter;
use crate::lang::{error::parser::Error, parser::{block, types}, token::Token, typed::types::Type, Spanned};
use super::block::Block;

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub ident: String,
    pub params: Vec<Spanned<(String, Type)>>,
    pub retrn_type: Spanned<Type>,
    pub body: Spanned<Block>,
}

/// Parses a function definition (given that the `fn` token has already been consumed)
pub fn parse_fn(tokens: &mut SpannedIter<'_, Token>) -> Result<Spanned<FuncDef>, Vec<KError<Error>>> {
    let start_span = tokens.span();

    // get the function identifier
    let ident = match tokens.next() {
        Some((Ok(Token::Ident(ident)), _)) => ident,
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),

        // un-recognised token
        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedFnIdent { ctx_span: start_span })]),
    };

    // parse the function parameters (if there are any)
    let (params, next_tok) = match tokens.next() {
        Some((Ok(Token::LParen), _)) => (Some(parse_fn_params(tokens)?), tokens.next()),

        // un-recognised token
        token => (None, token),
    };

    // parse the function return type
    let retrn_type = match next_tok {
        Some((Ok(Token::Arrw), _)) => types::parse_type(tokens.next(), tokens)?,

        // un-recognised token
        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedFnRetrnType { ctx_span: start_span.start..tokens.span().end })]),
    };

    // parse the function body
    let body = match tokens.next() {
        Some((Ok(Token::LBrace), _)) => block::parse_block(tokens)?,
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),

        // un-recognised token
        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedFnBody { ctx_span: start_span.start..tokens.span().end })])
    };

    // return the completed function definition
    Ok((
        FuncDef {
            ident,
            params: params.unwrap_or_default(),
            retrn_type,
            body,
        },
        start_span.start..tokens.span().end
    ))
}

/// Parses a single function parameter
fn parse_fn_param(params_span: Span, first_tok: Token, tokens: &mut SpannedIter<'_, Token>) -> Result<Spanned<(String, Type)>, Vec<KError<Error>>> {
    let start_span = tokens.span();

    // parse the identifier
    let Token::Ident(ident) = first_tok
    else {
        return Err(vec![KError::Other(tokens.span(), Error::ExpectedFnParamIdent { ctx_span: params_span })]);
    };

    // parse the colon
    match tokens.next() {
        Some((Ok(Token::Colon), _)) => (),
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),

        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedFnParamColon { ctx_span: start_span.start..tokens.span().end })]),
    }

    // parse the type
    let ptype = types::parse_type(tokens.next(), tokens)?;

    Ok((
        (ident, ptype.0),
        start_span.start..ptype.1.end,
    ))
}

/// Parses function parameters (assuming that the `(` token has already been consumed)
fn parse_fn_params(tokens: &mut SpannedIter<'_, Token>) -> Result<Vec<Spanned<(String, Type)>>, Vec<KError<Error>>> {
    let start_span = tokens.span();
    let mut params = Vec::new();

    // get the first parameter and also check for `)` for empty parameters
    let param = match tokens.next() {
        // empty parameters
        Some((Ok(Token::RParen), _)) => return Ok(params),

        // first parameter
        Some((Ok(token), _)) => parse_fn_param(start_span.clone(), token, tokens)?,
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),

        None => return Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses { ctx_span: start_span })]), // if there is never a `)`
    };
    params.push(param);

    while let Some((token, span)) = tokens.next() {
        match token {
            Ok(Token::RParen) => return Ok(params), // when the parameter is terminated
            Ok(Token::Comma) => {
                match tokens.next() {
                    Some((Ok(Token::RParen), _)) => return Ok(params),
                    Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
                    None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedCommaOrRParen { ctx_span: start_span })]),

                    Some((Ok(token), span)) => params.push(parse_fn_param(start_span.start..span.end, token, tokens)?),
                }
            },
            _ => return Err(vec![KError::Other(span, Error::ExpectedCommaOrRParen { ctx_span: start_span })]),
        }
    }

    // this section of code can only be reached when the parameter is never terminated with `)`
    Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses { ctx_span: start_span })])
}
