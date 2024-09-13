use json::{object, JsonValue};
use crate::scratch::call_procedure;

use super::{parse_expr, Expr};

/// A statement in scratch (doesn't return anything)
#[derive(Debug, Clone)]
pub enum Statement {
    // If {
    //     condition: Condition,
    //     body: Vec<Statement>,
    //     /// else
    //     otherwise: Option<Vec<Statement>>,
    // },

    CallProcedure {
        ident: String,
    },

    Ask { prompt: Expr },
    
    SetVar {
        ident: String,
        value: Expr,
    },
    ShowVar { ident: String },
    HideVar { ident: String },

    PushList {
        ident: String,
        value: Expr,
    },
    RemoveList {
        ident: String,
        idx: Expr,
    },
    InsertList {
        ident: String,
        value: Expr,
        idx: Expr,
    },
    ReplaceList {
        ident: String,
        value: Expr,
        idx: Expr,
    },
    ShowList { ident: String },
    HideList { ident: String },
    ClearList { ident: String },
}

/// Parses a scratch statement and outupts the generated json
pub(super) fn parse_stmt(stmt: &Statement, expr_blocks: &mut Vec<JsonValue>) -> JsonValue {
    use Statement as S;

    match stmt {
        S::CallProcedure { ident } => call_procedure(ident),
        S::PushList { ident, value } => {
            object! {
                opcode: "data_addtolist",
                next: null, // gets replaced later
                parent: null,
                inputs: {
                    ITEM: [
                        1,
                        parse_expr(value.clone(), expr_blocks),
                    ],
                },
                fields: {
                    LIST: [
                        **ident,
                        ""
                    ],
                },
                shadow: false,
                topLevel: false,
            }
        },
        S::ClearList { ident } => {
            object! {
                opcode: "data_deletealloflist",
                inputs: {},
                fields: {
                    LIST: [
                        **ident,
                        ""
                    ],
                },
            }
        },
        S::SetVar { ident, value } => {
            object! {
                opcode: "data_setvariableto",
                inputs: {
                    VALUE: [
                        1,
                        parse_expr(value.clone(), expr_blocks),
                    ],
                },
                fields: {
                    VARIABLE: [
                        **ident,
                        ""
                    ],
                },
            }
        },
        S::Ask { prompt } => {
            object! {
                opcode: "sensing_askandwait",
                inputs: {
                    QUESTION: [
                        1,
                        parse_expr(prompt.clone(), expr_blocks),
                    ],
                },
                fields: {},
            }
        },
        _ => todo!(),
    }
}
