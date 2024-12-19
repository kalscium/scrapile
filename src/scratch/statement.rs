use json::{object, JsonValue};
use crate::scratch::{block::parse_block, call_procedure, parse_cond};
use super::{parse_expr, Condition, Expr};

/// A statement in scratch (doesn't return anything)
#[derive(Debug, Clone)]
pub enum Statement {
    CallProcedure {
        ident: String,
    },

    Ask { prompt: Expr },
    
    SetVar {
        ident: String,
        value: Expr,
    },

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
    ClearList { ident: String },

    StopAll,

    If {
        condition: Condition,
        body: Vec<Statement>,
    },
    IfElse {
        condition: Condition,
        body: Vec<Statement>,
        otherwise: Vec<Statement>,
    },

    RepeatUntil {
        condition: Condition,
        body: Vec<Statement>,
    },
}
 /// Parses a scratch statement and outupts the generated json
pub(super) fn parse_stmt(stmt: Statement, expr_blocks: &mut Vec<JsonValue>) -> JsonValue {
    use Statement as S;

    match stmt {
        S::CallProcedure { ident } => call_procedure(&ident),
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
                        ident,
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
                        ident,
                        ""
                    ],
                },
            }
        },
        S::InsertList { ident, value, idx } => {
            object! {
                opcode: "data_insertatlist",
                inputs: {
                    ITEM: [
                        1,
                        parse_expr(value.clone(), expr_blocks),
                    ],
                    INDEX: [
                        1,
                        parse_expr(idx.clone(), expr_blocks),
                    ],
                },
                fields: {
                    LIST: [
                        ident,
                        "",
                    ],
                },
            }
        },
        S::ReplaceList { ident, value, idx } => {
            object! {
                opcode: "data_replaceitemoflist",
                inputs: {
                    ITEM: [
                        1,
                        parse_expr(value.clone(), expr_blocks),
                    ],
                    INDEX: [
                        1,
                        parse_expr(idx.clone(), expr_blocks),
                    ],
                },
                fields: {
                    LIST: [
                        ident,
                        "",
                    ],
                },
            }
        },
        S::RemoveList { ident, idx } => {
            object! {
                opcode: "data_deleteoflist",
                inputs: {
                    INDEX: [
                        1,
                        parse_expr(idx.clone(), expr_blocks),
                    ],
                },
                fields: {
                    LIST: [
                        ident,
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
                        ident,
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
        S::StopAll => {
            object! {
                opcode: "control_stop",
                inputs: {},
                fields: {
                    STOP_OPTION: [
                        "all",
                        null,
                    ],
                },
            }
        },
        S::If { condition, body } => {
            let condition = parse_cond(condition, expr_blocks);
            let body = parse_block(body, expr_blocks);

            object! {
                opcode: "control_if",
                inputs: {
                    CONDITION: [
                        1,
                        condition,
                    ],
                    SUBSTACK: [
                        1,
                        body,
                    ],
                },
                fields: {},
            }
        },
        S::IfElse { condition, body, otherwise } => {
            let condition = parse_cond(condition, expr_blocks);
            let body = parse_block(body, expr_blocks);
            let otherwise = parse_block(otherwise, expr_blocks);

            object! {
                opcode: "control_if_else",
                inputs: {
                    CONDITION: [
                        1,
                        condition,
                    ],
                    SUBSTACK: [
                        1,
                        body,
                    ],
                    SUBSTACK2: [
                        1,
                        otherwise,
                    ],
                },
                fields: {},
            }
        },
        S::RepeatUntil { condition, body } => {
            let condition = parse_cond(condition, expr_blocks);
            let body = parse_block(body, expr_blocks);

            object! {
                opcode: "control_repeat_until",
                inputs: {
                    CONDITION: [
                        1,
                        condition,
                    ],
                    SUBSTACK: [
                        1,
                        body,
                    ],
                },
                fields: {},
            }
        },
    }
}
