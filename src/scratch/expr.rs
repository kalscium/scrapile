use json::{array, object, JsonValue};
use crate::scratch::{expr_idx_to_id, parse_cond};

use super::Condition;

/// An expression in scratch (returns a value)
#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    Float(f64),
    PosFloat(f64),
    PosInteger(u32),
    Integer(i32),
    String(String),

    // operations
    Condition(Box<Condition>), // conditions can be converted to strings as an expr
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Concat(Box<Expr>, Box<Expr>),

    // Asking
    Answer,
    Timer,

    // list & variable operations
    Variable { ident: String },
    ListElement {
        ident: String,
        idx: Box<Expr>,
    },
    ListLength { ident: String },

    // string operations
    StringElement {
        string: Box<Expr>,
        idx: Box<Expr>,
    },
    StringLength { string: Box<Expr> },
}

/// Parses a scratch expression and outputs the generated json
/// (requires a mutable reference to the block vector to add addtional blocks for multi-step exressions)
pub(super) fn parse_expr(expr: Expr, expr_blocks: &mut Vec<JsonValue>) -> JsonValue {
    use Expr as E;

    match expr {
        // basic numbers & strings
        E::Float(num) => array![ 4, num ],
        E::PosFloat(num) => array![ 5, num ],
        E::PosInteger(num) => array![ 6, num ],
        E::Integer(num) => array![ 7, num ],
        E::String(num) => array![ 10, num ],

        // concat
        E::Concat(lhs, rhs) => {
            let json = object! {
                opcode: "operator_join",
                next: null,
                parent: null,
                inputs: {
                    STRING1: [
                        1,
                        parse_expr((*lhs).clone(), expr_blocks),
                    ],
                    STRING2: [
                        1,
                        parse_expr((*rhs).clone(), expr_blocks),
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into() },

        // maths operations
        E::Add(lhs, rhs) => {
            let json = object! {
                opcode: "operator_add",
                next: null,
                parent: null,
                inputs: {
                    NUM1: [
                        1,
                        parse_expr((*lhs).clone(), expr_blocks),
                    ],
                    NUM2: [
                        1,
                        parse_expr((*rhs).clone(), expr_blocks),
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::Sub(lhs, rhs) => {
            let json = object! {
                opcode: "operator_subtract",
                next: null,
                parent: null,
                inputs: {
                    NUM1: [
                        1,
                        parse_expr((*lhs).clone(), expr_blocks),
                    ],
                    NUM2: [
                        1,
                        parse_expr((*rhs).clone(), expr_blocks),
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::Mul(lhs, rhs) => {
            let json = object! {
                opcode: "operator_multiply",
                next: null,
                parent: null,
                inputs: {
                    NUM1: [
                        1,
                        parse_expr((*lhs).clone(), expr_blocks),
                    ],
                    NUM2: [
                        1,
                        parse_expr((*rhs).clone(), expr_blocks),
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::Div(lhs, rhs) => {
            let json = object! {
                opcode: "operator_divide",
                next: null,
                parent: null,
                inputs: {
                    NUM1: [
                        1,
                        parse_expr((*lhs).clone(), expr_blocks),
                    ],
                    NUM2: [
                        1,
                        parse_expr((*rhs).clone(), expr_blocks),
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::Mod(lhs, rhs) => {
            let json = object! {
                opcode: "operator_mod",
                next: null,
                parent: null,
                inputs: {
                    NUM1: [
                        1,
                        parse_expr((*lhs).clone(), expr_blocks),
                    ],
                    NUM2: [
                        1,
                        parse_expr((*rhs).clone(), expr_blocks),
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::Condition(cond) => parse_cond(*cond, expr_blocks),

        // asking
        E::Answer => {
            let json = object! {
                opcode: "sensing_answer",
                next: null,
                parent: null,
                inputs: {},
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::Timer => {
            let json = object! {
                opcode: "sensing_timer",
                next: null,
                parent: null,
                inputs: {},
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },

        // variables and lists
        E::Variable { ident } => array![ 12, ident, "" ],
        E::ListElement { ident, idx } => {
            let json = object! {
                opcode: "data_itemoflist",
                next: null,
                parent: null,
                inputs: {
                    INDEX: [
                        1,
                        parse_expr((*idx).clone(), expr_blocks),
                    ]
                },
                fields: {
                    LIST: [
                        ident,
                        "",
                    ]
                },
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::ListLength { ident } => {
            let json = object! {
                opcode: "data_lengthoflist",
                next: null,
                parent: null,
                inputs: {},
                fields: {
                    LIST: [
                        ident,
                        "",
                    ],
                },
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        }

        // string operations
        E::StringElement { string, idx } => {
            let json = object! {
                opcode: "operator_letter_of",
                next: null,
                parent: null,
                inputs: {
                    LETTER: [
                        1,
                        parse_expr((*idx).clone(), expr_blocks),
                    ],
                    STRING: [
                        1,
                        parse_expr((*string).clone(), expr_blocks),
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        E::StringLength { string } => {
            let json = object! {
                opcode: "operator_length",
                next: null,
                parent: null,
                inputs: {
                    STRING: [
                        1,
                        parse_expr((*string).clone(), expr_blocks),
                    ],
                },
                fields: {},
            };
            expr_blocks.push(json);

            expr_idx_to_id(expr_blocks.len()-1).into()
        }
    }
}
