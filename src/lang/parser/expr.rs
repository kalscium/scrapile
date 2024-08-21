use ketchup::{error::KError, node::Node, parser::Parser, OperInfo, Space, Span};
use logos::SpannedIter;
use crate::lang::{error::parser::Error, token::Token, Spanned};
use super::block::Block;

#[derive(Debug, Clone)]
pub struct Expr {
    pub span: Span,
    pub asa: Vec<Node<ExprOper>>,
}

#[derive(Debug, Clone)]
pub enum ExprOper {
    Integer(u32),
    Float(f64),
    String(String),
    Ident(String),

    Add,
    Sub,
    Mul,
    Div,
    Concat,
    DotAccess,

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
    Call(String, Vec<Expr>),
    BuiltinFnCall(String, Vec<Expr>),
    Block(Block),
}

#[inline]
pub fn parse_expr(first_tok: Option<Spanned<Result<Token, Error>>>, tokens: &mut SpannedIter<'_, Token>) -> Result<(Expr, Option<Spanned<Token>>), Vec<KError<Error>>> {
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

fn parse_call_or_ident(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<(OperInfo<ExprOper>, Option<Spanned<Result<Token, Error>>>), Vec<KError<Error>>> {
    let start_span = tokens.span();
    let next_tok = tokens.next();

    // check for function call and handle errors
    if let Some((token, span)) = next_tok {
        let token = token.map_err(|err| vec![KError::Other(span.clone(), err)])?;
        
        if token == Token::LParen {
            // if there is a function call then parse the arguments of it
            let (args, span) = super::tuple::parse_tuple(tokens)?;
            return Ok((OperInfo {
                oper: ExprOper::Call(ident, args),
                span: start_span.start..span.end,
                space: Space::None,
                precedence: 0,
            }, tokens.next()));
        } else {
            // if the token found is not of a function call (`LParen`)
            return Ok((OperInfo {
                oper: ExprOper::Ident(ident),
                span: start_span,
                space: Space::None,
                precedence: 0,
            }, Some((Ok(token), span))));
        }
    }

    // if there are no tokens following the identifier
    Ok((OperInfo {
        oper: ExprOper::Ident(ident),
        span: start_span,
        space: Space::None,
        precedence: 0,
    }, tokens.next()))
}

fn parse_tuple(tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<ExprOper>, Option<Spanned<Result<Token, Error>>>)>, Vec<KError<Error>>> {
    let (tuple, span) = super::tuple::parse_tuple(tokens)?;

    Ok(Some((OperInfo {
        oper: ExprOper::Tuple(tuple),
        span,
        space: Space::None,
        precedence: 0,
    }, tokens.next())))
}

fn parse_block(tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<ExprOper>, Option<Spanned<Result<Token, Error>>>)>, Vec<KError<Error>>> {
    let (block, span) = super::block::parse_block(tokens)?;

    Ok(Some((OperInfo {
        oper: ExprOper::Block(block),
        span,
        space: Space::None,
        precedence: 0,
    }, tokens.next())))
}

fn parse_builtin_func_call(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<ExprOper>, Option<Spanned<Result<Token, Error>>>)>, Vec<KError<Error>>> {
    let start_span = tokens.span();

    // check for args for the builtin func call and also handle errors
    match tokens.next() {
        Some((Ok(Token::LParen), _)) => {
            let (args, span) = super::tuple::parse_tuple(tokens)?;
            Ok(Some((OperInfo {
                oper: ExprOper::BuiltinFnCall(ident, args),
                span: start_span.start..span.end,
                space: Space::None,
                precedence: 0,
            }, tokens.next())))
        },
        token => {
            // otherwise return a builtin function call without args
            Ok(Some((OperInfo {
                oper: ExprOper::BuiltinFnCall(ident, Vec::new()),
                span: start_span,
                space: Space::None,
                precedence: 0,
            }, token)))
        },
    }

}

fn oper_generator(token: Token, tokens: &mut SpannedIter<'_, Token>, double_space: bool) -> Result<Option<(OperInfo<ExprOper>, Option<Spanned<Result<Token, Error>>>)>, Vec<KError<Error>>> {
    use Token as T;
    use ExprOper as E;

    let (precedence, space, oper) = match (token, double_space) {
        // literals
        (T::Integer(int), _) => (0, Space::None, E::Integer(int)),
        (T::Float(f), _) => (0, Space::None, E::Float(f)),
        (T::String(str), _) => (0, Space::None, E::String(str)),

        // identifiers
        (T::Ident(ident), _) => return parse_call_or_ident(ident, tokens).map(|(info, next)| Some((info, next))),
        (T::BuiltinFunc(ident), _) => return parse_builtin_func_call(ident, tokens),

        // dot memeber access
        (T::Dot, _) => (1, Space::Double, E::DotAccess),

        // single space (negative, postive & not)
        (T::Plus, false) => (2, Space::Single, E::Pos),
        (T::Minus, false) => (2, Space::Single, E::Neg),
        (T::Not, false) => (2, Space::Single, E::Not),

        // multiplication & division
        (T::Star, _) => (3, Space::Double, E::Mul),
        (T::Slash, _) => (3, Space::Double, E::Div),

        // addition & subtraction
        (T::Plus, true) => (4, Space::Double, E::Add),
        (T::Minus, true) => (4, Space::Double, E::Sub),

        // concatination
        (T::Concat, _) => (5, Space::Double, E::Concat),

        // comparisions
        (T::EE, _) => (6, Space::Double, E::EE),
        (T::NE, _) => (6, Space::Double, E::NE),
        (T::GT, _) => (6, Space::Double, E::GT),
        (T::LT, _) => (6, Space::Double, E::LT),
        (T::GTE, _) => (6, Space::Double, E::GTE),
        (T::LTE, _) => (6, Space::Double, E::LTE),

        // and, or
        (T::And, _) => (7, Space::Double, E::And),
        (T::Or, _) => (7, Space::Double, E::Or),

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
