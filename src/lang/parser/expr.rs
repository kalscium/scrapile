use ketchup::{error::KError, node::Node, parser::Parser, OperInfo, Space, Span};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(u32),
    Float(f64),
    String(String),
    Ident(Vec<String>),

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
pub fn parse_expr(tokens: &mut SpannedIter<'_, Token>) -> Result<(Vec<Node<Expr>>, Option<(Token, Span)>), Vec<KError<Token, Error>>> {
    Parser::<'_, Token, Expr, _, Vec<Node<Expr>>, _, Error>::new(tokens, oper_generator).parse()
}

fn parse_ident(ident: String, tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<Expr>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Token, Error>>> {
    let ((ident, span), next_tok) = super::ident::parse_ident(ident, tokens)?;

    Ok(Some((OperInfo {
        oper: Expr::Ident(ident),
        span,
        space: Space::None,
        precedence: 0,
    }, next_tok.map(|(tok, span)| (Ok(tok), span)))))
}

fn parse_paren(tokens: &mut SpannedIter<'_, Token>) -> Result<Option<(OperInfo<Expr>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Token, Error>>> {
    let start_span = tokens.span();
    let (asa, next_tok) = parse_expr(tokens)?;

    // ensure the closing parentheses is found
    match next_tok {
        Some((Token::RParen, _)) => (), // okay
        _ => return Err(vec![KError::Other(tokens.span(), Error::UnclosedParentheses)]),
    }

    Ok(Some((OperInfo {
        oper: Expr::Scope(asa),
        span: start_span.start..tokens.span().end,
        space: Space::None,
        precedence: 0,
    }, tokens.next())))
}

fn oper_generator(token: Token, tokens: &mut SpannedIter<'_, Token>, double_space: bool) -> Result<Option<(OperInfo<Expr>, Option<(Result<Token, Error>, Span)>)>, Vec<KError<Token, Error>>> {
    use Token as T;
    use Expr as E;

    let (precedence, space, oper) = match (token, double_space) {
        // literals
        (T::Integer(int), _) => (0, Space::None, E::Integer(int)),
        (T::Float(f), _) => (0, Space::None, E::Float(f)),
        (T::String(str), _) => (0, Space::None, E::String(str)),

        // identifiers
        (T::Ident(ident), _) => return parse_ident(ident, tokens),

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
        (T::LParen, _) => return parse_paren(tokens),

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
