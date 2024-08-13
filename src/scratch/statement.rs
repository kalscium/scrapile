use json::{object, JsonValue};
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
    ClearList(String),
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
}

/// Parses a scratch statement and outupts the generated json
pub(super) fn parse_stmt(stmt: &Statement, expr_blocks: &mut Vec<JsonValue>) -> JsonValue {
    use Statement as S;

    match stmt {
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
        _ => todo!(),
    }
}
