use ketchup::{error::KError, node::Node, parser::Parser, OperInfo, Space, Span};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};
use super::{block::Block, ident::Call};

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub asa: Vec<Node<ExprOper>>,
}

#[derive(Debug, Clone)]
pub enum ExprOper { Integer(u32),
    Float(f64),
    String(String),
    Ident(Vec<String>),

    Add,
    Sub,
    Mul,
    Div,
    Concat,

    Neg,
    Pos,
    Not,

    Or,
    And,
    EE,
    NE,
    GT,
    LT,
    GTE,
    LTE,

    Tuple(Vec<Expr>),
    Call(Call),
    Block(Block),
}

#[inline]
pub fn parse_expr(first_tok: Option<(Result<Token, Error>, Span)>, tokens: &mut SpannedIter<'_, Token>) -> Result<(Expr, Option<(Token, Span)>), Vec<KError<Error>>> {
    let start_span = tokens.span();
    let (asa, next_tok) = Parser::<'_, Token, ExprOper, _, Vec<Node<ExprOper>>, _, Error>::new(tokens, oper_generator).parse(first_tok)?;

    let span_start = asa.first().map(|node| node.info.span.start).unwrap_or(start_span.start);
    let span_end = asa.last().map(|node| node.info.span.end).unwrap_or(start_span.end);

    // ensure that the expr is not empty
    if asa.is_empty() {
        return Err(vec![KError::Other(span_start..span_end, Error::ExpectedExpr)]);
    }

    Ok((Expr {
        span: span_start..span_end,
        asa,
    }, next_tok))
}

fn parse_call_or_ident(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<ExprOper>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Error>>> {
    let ((oper, span), next_tok) = super::ident::parse_call_or_ident(ident, tokens)?;

    Ok(Some((OperInfo {
        oper,
        span,
        space: Space::None,
        precedence: 0,
    }, next_tok)))
}

fn parse_tuple(tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<ExprOper>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Error>>> {
    let (tuple, span) = super::tuple::parse_tuple(tokens)?;

    Ok(Some((OperInfo {
        oper: ExprOper::Tuple(tuple),
        span,
        space: Space::None,
        precedence: 0,
    }, tokens.next())))
}

fn parse_block(tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<ExprOper>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Error>>> {
    let (block, span) = super::block::parse_block(tokens)?;

    Ok(Some((OperInfo {
        oper: ExprOper::Block(block),
        span,
        space: Space::None,
        precedence: 0,
    }, tokens.next())))
}

fn parse_builtin_func_call(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<ExprOper>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Error>>> {
    let ((ident, start_span), next_tok) = super::ident::parse_ident(ident, tokens)?;

    // check for args for the builtin func call
    if let Some((Token::LParen, _)) = &next_tok {
        let (args, span) = super::tuple::parse_tuple(tokens)?;
        return Ok(Some((OperInfo {
            oper: ExprOper::Call(Call { ident, args, is_builtin: true }),
            span: start_span.start..span.end,
            space: Space::None,
            precedence: 0,
        }, tokens.next())));
    }

    // otherwise return a builtin function call without args
    return Ok(Some((OperInfo {
        oper: ExprOper::Call(Call { ident, args: Vec::new(), is_builtin: true }),
        span: start_span,
        space: Space::None,
        precedence: 0,
    }, next_tok.map(|(tok, span)| (Ok(tok), span)))));
}

fn oper_generator(token: Token, tokens: &mut SpannedIter<'_, Token>, double_space: bool) -> Result<Option<(OperInfo<ExprOper>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Error>>> {
    use Token as T;
    use ExprOper as E;

    let (precedence, space, oper) = match (token, double_space) {
        // literals
        (T::Integer(int), _) => (0, Space::None, E::Integer(int)),
        (T::Float(f), _) => (0, Space::None, E::Float(f)),
        (T::String(str), _) => (0, Space::None, E::String(str)),

        // identifiers
        (T::Ident(ident), _) => return parse_call_or_ident(ident, tokens),
        (T::BuiltinFunc(ident), _) => return parse_builtin_func_call(ident, tokens),

        // single space
        (T::Plus, false) => (1, Space::Single, E::Pos),
        (T::Minus, false) => (1, Space::Single, E::Neg),
        (T::Not, false) => (1, Space::Single, E::Not),

        // multiplication & division
        (T::Star, _) => (2, Space::Double, E::Mul),
        (T::Slash, _) => (2, Space::Double, E::Div),

        // addition & subtraction
        (T::Plus, true) => (3, Space::Double, E::Add),
        (T::Minus, true) => (3, Space::Double, E::Sub),

        // concatination
        (T::Concat, _) => (4, Space::Double, E::Concat),

        // comparisions
        (T::EE, _) => (5, Space::Double, E::EE),
        (T::NE, _) => (5, Space::Double, E::NE),
        (T::GT, _) => (5, Space::Double, E::GT),
        (T::LT, _) => (5, Space::Double, E::LT),
        (T::GTE, _) => (5, Space::Double, E::GTE),
        (T::LTE, _) => (5, Space::Double, E::LTE),

        // and, or
        (T::And, _) => (6, Space::Double, E::And),
        (T::Or, _) => (6, Space::Double, E::Or),

        // tuples & blocks
        (T::LParen, _) => return parse_tuple(tokens),
        (T::LBrace, _) => return parse_block(tokens),

        // tokens this oper generator doesn't recognise
        _ => return Ok(None),
    };
    
    Ok(Some((OperInfo {
        oper,
        span: tokens.span(),
        space,
        precedence,
    }, tokens.next())))
}
