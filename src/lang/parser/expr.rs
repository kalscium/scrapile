use ketchup::{error::KError, node::Node, parser::Parser, OperInfo, Space};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(u32),
    Float(f64),
    String(String),

    Add,
    Sub,
    Mul,
    Div,

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

    Scope(Vec<Node<Expr>>),
}

#[inline]
pub fn parse_expr(tokens: &mut SpannedIter<'_, Token>, eof_token: Option<Token>) -> Result<Vec<Node<Expr>>, Vec<KError<Token, Error>>> {
    Parser::<'_, Token, Expr, _, Vec<Node<Expr>>, _, Error>::new(tokens, eof_token, oper_generator).parse()
}

fn parse_paren(tokens: &mut SpannedIter<'_, Token>) -> Result<OperInfo<Expr>, Vec<KError<Token, Error>>> {
    let start_span = tokens.span();
    let asa = parse_expr(tokens, Some(Token::RParen))?;

    Ok(OperInfo {
        oper: Expr::Scope(asa),
        span: start_span.start..tokens.span().end,
        space: Space::None,
        precedence: 0,
    })
}

fn oper_generator(token: Token, tokens: &mut SpannedIter<'_, Token>, double_space: bool) -> Result<OperInfo<Expr>, Vec<KError<Token, Error>>> {
    use Token as T;
    use Expr as E;

    let (precedence, space, oper) = match (token, double_space) {
        // literals
        (T::Integer(int), _) => (0, Space::None, E::Integer(int)),
        (T::Float(f), _) => (0, Space::None, E::Float(f)),
        (T::String(str), _) => (0, Space::None, E::String(str)),

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

        // comparisions
        (T::EE, _) => (4, Space::Double, E::EE),
        (T::NE, _) => (4, Space::Double, E::NE),
        (T::GT, _) => (4, Space::Double, E::GT),
        (T::LT, _) => (4, Space::Double, E::LT),
        (T::GTE, _) => (4, Space::Double, E::GTE),
        (T::LTE, _) => (4, Space::Double, E::LTE),

        // and, or
        (T::And, _) => (5, Space::Double, E::And),
        (T::Or, _) => (5, Space::Double, E::Or),

        // parentheses
        (T::RParen, _) => return Err(vec![KError::UnexpectedOper(tokens.span())]),
        (T::LParen, _) => return parse_paren(tokens),

        _ => todo!(),
    };
    
    Ok(OperInfo {
        oper,
        span: tokens.span(),
        space,
        precedence,
    })
}
