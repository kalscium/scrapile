use json::{object, JsonValue};

use crate::scratch::{expr_idx_to_id, parse_expr};

use super::Expr;

/// A condition in scratch (different from an expression as it can only be used in if statements)
#[derive(Debug, Clone)]
pub enum Condition {
    // expr to expr conditinos
    GreaterThan(Expr, Expr),
    LessThan(Expr, Expr),
    EqualTo(Expr, Expr),

    // condition to condition conditions
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

/// Parses a scratch condition and outputs the generated json
/// (requires a mutable reference to the block vector to add addtional blocks for multi-step conditions)
pub(super) fn parse_cond(cond: Condition, expr_blocks: &mut Vec<JsonValue>) -> JsonValue {
    use Condition as C;

    match cond {
        C::EqualTo(lhs, rhs) => {
            let lhs = parse_expr(lhs, expr_blocks);
            let rhs = parse_expr(rhs, expr_blocks);
            let json = object! {
                opcode: "operator_equals",
                next: null,
                parent: null,
                inputs: {
                    OPERAND1: [
                        1,
                        lhs,
                    ],
                    OPERAND2: [
                        1,
                        rhs,
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);
            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        C::GreaterThan(lhs, rhs) => {
            let lhs = parse_expr(lhs, expr_blocks);
            let rhs = parse_expr(rhs, expr_blocks);
            let json = object! {
                opcode: "operator_gt",
                next: null,
                parent: null,
                inputs: {
                    OPERAND1: [
                        1,
                        lhs,
                    ],
                    OPERAND2: [
                        1,
                        rhs,
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);
            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        C::LessThan(lhs, rhs) => {
            let lhs = parse_expr(lhs, expr_blocks);
            let rhs = parse_expr(rhs, expr_blocks);
            let json = object! {
                opcode: "operator_lt",
                next: null,
                parent: null,
                inputs: {
                    OPERAND1: [
                        1,
                        lhs,
                    ],
                    OPERAND2: [
                        1,
                        rhs,
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);
            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        C::And(lhs, rhs) => {
            let lhs = parse_cond(*lhs, expr_blocks);
            let rhs = parse_cond(*rhs, expr_blocks);
            let json = object! {
                opcode: "operator_and",
                next: null,
                parent: null,
                inputs: {
                    OPERAND1: [
                        1,
                        lhs,
                    ],
                    OPERAND2: [
                        1,
                        rhs,
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);
            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        C::Or(lhs, rhs) => {
            let lhs = parse_cond(*lhs, expr_blocks);
            let rhs = parse_cond(*rhs, expr_blocks);
            let json = object! {
                opcode: "operator_or",
                next: null,
                parent: null,
                inputs: {
                    OPERAND1: [
                        1,
                        lhs,
                    ],
                    OPERAND2: [
                        1,
                        rhs,
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);
            expr_idx_to_id(expr_blocks.len()-1).into()
        },
        C::Not(cond) => {
            let cond = parse_cond(*cond, expr_blocks);
            let json = object! {
                opcode: "operator_not",
                next: null,
                parent: null,
                inputs: {
                    OPERAND: [
                        1,
                        cond,
                    ],
                },
                fields: {},
                shadow: false,
                topLevel: false,
            };
            expr_blocks.push(json);
            expr_idx_to_id(expr_blocks.len()-1).into()
        },
    }
}
