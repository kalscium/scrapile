use logos::Logos;
use crate::lang::error::parser::Error;

/// A token generated by the lexer
#[derive(Debug, Clone, Logos, PartialEq)]
#[logos(error = Error)]
#[logos(skip r"[ \t\r\n\f]+")] // whitespace
#[logos(skip r"\/\/[^\n\r]*")] // `//` comments
#[logos(skip r"#[^\n\r]*")] // `#` comments
#[logos(skip r"\/\*[^\*\/]*\*\/")] // multi-line comments
pub enum Token {
    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    #[regex(r"[0-9]+\.[0-9]+f", |lex| lex.slice()[1..lex.slice().len()-1].parse::<f64>().unwrap())]
    Number(f64),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_owned())]
    String(String),
    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Bool(bool),

    // Symbols
    #[token(".")]
    Dot,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token("!")]
    Not,
    #[token(",")]
    Comma,
    #[token("<>")]
    Concat,

    // Maths operations
    #[token("+")]
    Plus,
    #[token("+=")]
    AddEq,
    #[token("*")]
    Star,
    #[token("*=")]
    MulEq,
    #[token("/")]
    Slash,
    #[token("/=")]
    DivEq,
    #[token("%")]
    Modulo,
    #[token("%=")]
    ModuloEq,

    // `-` symbols
    #[token("-")]
    Minus,
    #[token("-=")]
    SubEq,
    #[token("->")]
    Arrw,

    // Comparisions
    #[token(">")]
    GT,
    #[token("<")]
    LT,
    #[token(">=")]
    GTE,
    #[token("<=")]
    LTE,

    // `=` symbols
    #[token("=")]
    EQ,
    #[token("==")]
    EE,
    #[token("!=")]
    NE,

    // Parentheses
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,   

    // Brackets
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    // Braces
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    // Keywords & Idents
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_-]*", |lex| lex.slice().to_owned())]
    Ident(String),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_-]*!", |lex| lex.slice()[..lex.slice().len()-1].to_owned())]
    BuiltinFunc(String),
    #[token("main")]
    Main,
    #[token("var")]
    Var,
    #[token("mut")]
    Mut,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("fn")]
    Func,
}

