use ketchup::node::Node;
use crate::lang::{error::typed::Error, parser::{block::Block, expr::ExprOper}, typed::types::Type, Spanned};
use super::{symbol_table::TypeTable, types::Typed};

/// A tree version of expr for type annotation
#[derive(Debug)]
pub enum TExpr {
    Number(f64),
    String(String),
    Ident(String),

    Add(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    Sub(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    Mul(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    Div(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    Concat(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),

    DotAccess(u32),

    Neg(Box<Typed<Spanned<TExpr>>>),
    Pos(Box<Typed<Spanned<TExpr>>>),
    Not(Box<Typed<Spanned<TExpr>>>),

    Or,
    And,
    EE,
    NE,
    GT,
    LT,
    GTE,
    LTE,

    Tuple(Vec<Typed<Spanned<TExpr>>>),
    Call(String, Vec<Typed<Spanned<TExpr>>>),
    BuiltinFnCall(String, Vec<Vec<Typed<Spanned<TExpr>>>>),
    Block(Typed<Spanned<Block>>),
}

/// Wraps an expr with types and also returns it's current location in the asa
pub fn wrap_expr(asa: &[Node<ExprOper>], _symbol_table: &TypeTable) -> Result<(Typed<Spanned<TExpr>>, usize), Error> {
    use ExprOper as EO;

    Ok(match &asa[0].oper {
        // literals
        EO::Number(num) => (((TExpr::Number(*num), asa[0].info.span.clone()), Type::Number), 0),
        EO::String(string) => (((TExpr::String(string.clone()), asa[0].info.span.clone()), Type::String), 0),

        // negative & positive
        EO::Neg => {
            // wrap the sub-expr that this negates
            let (expr, idx) = wrap_expr(&asa[1..], _symbol_table)?;

            // make sure it's a number, otherwise throw error
            if expr.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "negate",
                    value_span: expr.0.1,
                    value_type: expr.1,
                });
            }

            // return negated value
            let expr_span = expr.0.1.clone();
            (
                (
                    (
                        TExpr::Neg(Box::new(expr)), // value
                        asa[0].info.span.start..expr_span.end, // span
                    ),
                    Type::Number, // type
                ),
                idx + 1, // idx (+1 as that's the offset given to the wrapper that produced it)
            )
        },
        EO::Pos => {
            // wrap the sub-expr that this does nothing to
            let (expr, idx) = wrap_expr(&asa[1..], _symbol_table)?;

            // make sure it's a number, otherwise throw error
            if expr.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "pos",
                    value_span: expr.0.1,
                    value_type: expr.1,
                });
            }

            // return negated value
            let span = asa[0].info.span.start..expr.0.1.end;
            (
                (
                    (
                        TExpr::Neg(Box::new(expr)), // value
                        span, // span
                    ),
                    Type::Number, // type
                ),
                idx + 1, // idx (+1 as that's the offset given to the wrapper that produced it)
            )
        },

        // arithmatic / maths
        EO::Add => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], _symbol_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], _symbol_table)?;

            // make sure lhs is of type number, otherwise throw error
            if lhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "add",
                    value_span: lhs.0.1.clone(),
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type number, otherwise throw error
            if rhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "add",
                    value_span: rhs.0.1.clone(),
                    value_type: rhs.1,
                });
            }

            // return typed add operation
            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::Add(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Number, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::Sub => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], _symbol_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], _symbol_table)?;

            // make sure lhs is of type number, otherwise throw error
            if lhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "subtract",
                    value_span: lhs.0.1.clone(),
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type number, otherwise throw error
            if rhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "subtract",
                    value_span: rhs.0.1.clone(),
                    value_type: rhs.1,
                });
            }

            // return typed add operation
            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::Sub(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Number, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::Mul => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], _symbol_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], _symbol_table)?;

            // make sure lhs is of type number, otherwise throw error
            if lhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "multiply",
                    value_span: lhs.0.1.clone(),
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type number, otherwise throw error
            if rhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "multiply",
                    value_span: rhs.0.1.clone(),
                    value_type: rhs.1,
                });
            }

            // return typed add operation
            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::Mul(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Number, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::Div => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], _symbol_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], _symbol_table)?;

            // make sure lhs is of type number, otherwise throw error
            if lhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "divide",
                    value_span: lhs.0.1.clone(),
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type number, otherwise throw error
            if rhs.1 != Type::Number {
                return Err(Error::ArithmeticNonNumber {
                    oper_span: asa[0].info.span.clone(),
                    oper_type: "divide",
                    value_span: rhs.0.1.clone(),
                    value_type: rhs.1,
                });
            }

            // return typed add operation
            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::Div(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Number, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },

        // A change of pace where only strings are allowed instead of numbers
        EO::Concat => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], _symbol_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], _symbol_table)?;

            // make sure lhs is of type string, otherwise throw error
            if lhs.1 != Type::String {
                return Err(Error::ConcatNonString {
                    oper_span: asa[0].info.span.clone(),
                    value_span: lhs.0.1.clone(),
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type number, otherwise throw error
            if rhs.1 != Type::String {
                return Err(Error::ConcatNonString {
                    oper_span: asa[0].info.span.clone(),
                    value_span: rhs.0.1.clone(),
                    value_type: rhs.1,
                });
            }

            // return typed add operation
            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::Div(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Number, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },

        _ => todo!(),
    })
}
