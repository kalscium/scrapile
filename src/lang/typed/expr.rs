use ketchup::node::Node;
use crate::lang::{error::typed::Error, parser::expr::ExprOper, typed::{block, builtin, types::Type}, Spanned};
use super::{block::TBlock, builtin::TBuiltinFnCall, symbol_table::{TypeTable, VarTable}, types::Typed};

/// A tree version of an expr for type annotation
#[derive(Debug)]
pub enum TExpr {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,

    Add(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>), Sub(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    Mul(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    Div(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    Concat(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),

    Neg(Box<Typed<Spanned<TExpr>>>),
    Pos(Box<Typed<Spanned<TExpr>>>),
    Not(Box<Typed<Spanned<TExpr>>>),

    Or(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    And(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    EE(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    NE(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    GT(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    LT(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    GTE(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),
    LTE(Box<Typed<Spanned<TExpr>>>, Box<Typed<Spanned<TExpr>>>),

    Tuple(Vec<Typed<Spanned<TExpr>>>),
    Call(String, Vec<Typed<Spanned<TExpr>>>),
    BuiltinFnCall(Box<TBuiltinFnCall>),
    Block(Box<TBlock>),
    VarGet {
        ident: String,
        var_type: Type,
    },
    List(Type, Vec<Spanned<TExpr>>),
}

/// Wraps an expr with types and also returns it's current location in the asa
pub fn wrap_expr(asa: &[Node<ExprOper>], type_table: &TypeTable, var_table: &mut VarTable) -> Result<(Typed<Spanned<TExpr>>, usize), Error> {
    use ExprOper as EO;

    Ok(match &asa[0].oper {
        // literals
        EO::Number(num) => (((TExpr::Number(*num), asa[0].info.span.clone()), Type::Number), 0),
        EO::String(string) => (((TExpr::String(string.clone()), asa[0].info.span.clone()), Type::String), 0),
        EO::Bool(bool) => (((TExpr::Bool(*bool), asa[0].info.span.clone()), Type::Bool), 0),
        EO::Nil => (((TExpr::Nil, asa[0].info.span.clone()), Type::Nil), 0),

        // blocks
        EO::Block(block) => {
            let (block, block_type) = block::wrap_block(block.clone(), type_table, var_table.spawn_scope())?;
            (
                (
                    (
                        TExpr::Block(Box::new(block)),
                        asa[0].info.span.clone(),                    
                    ),
                    block_type,
                ),
                0
            )
        },

        // tuples definitions
        EO::Tuple(exprs) => {
            let mut types = Vec::new();
            let mut typed_exprs = Vec::new();

            // append the types stored in the tuple
            for expr in exprs {
                let ((expr, ttype), _) = wrap_expr(&expr.asa, type_table, var_table)?;
                types.push(ttype.clone());
                typed_exprs.push((expr, ttype));
            }

            // if the tuple is empty, return a nill insetad
            if typed_exprs.is_empty() {
                return Ok((((TExpr::Nil, asa[0].info.span.clone()), Type::Nil), 0));
            }

            // if the tuple only has one member, expand it
            if typed_exprs.len() == 1 {
                return Ok((typed_exprs.pop().unwrap(), 0));
            }

            // return tuple
            (
                (
                    (
                        TExpr::Tuple(typed_exprs),
                        asa[0].info.span.clone(),
                    ),
                    Type::Tuple(types),
                ),
                0,
            )
        },

        // lists
        EO::List(list) => {
            // get the type of the first element in the list
            let (list_type, list_type_span, mut exprs) = match list.get(0) {
                Some(expr) => {
                    let ((expr, expr_type), _) = wrap_expr(&expr.asa, type_table, var_table)?;
                    (expr_type.clone(), expr.1.clone(), vec![expr])
                },
                None => (Type::Nil, asa[0].info.span.clone(), Vec::new()),
            };

            // wrap the rest of the list's elements and make sure they're all of the correct type
            for (i, expr) in list.into_iter().enumerate() {
                // skip the first expr as it's set already
                if i == 0 { continue };

                // wrap the expr make sure it's the right type
                let (((expr, expr_span), expr_type), _) = wrap_expr(&expr.asa, type_table, var_table)?;
                if expr_type != list_type {
                    return Err(Error::ListElementTypeMismatch { first_span: list_type_span, first_type: list_type, el_span: expr_span, el_type: expr_type });
                }

                // push it
                exprs.push((expr, expr_span));
            }
            
            // return list definition
            (
                (
                    (
                        TExpr::List(list_type.clone(), exprs),
                        asa[0].info.span.clone(),
                    ),
                    Type::List(Box::new(list_type))
                ),
                0,
            )
        },

        // builtin-function calls
        EO::BuiltinFnCall { ident, ident_span, args } => {
            let (call, ttype) = builtin::wrap_builtin(ident, ident_span.clone(), asa[0].info.span.clone(), &args, type_table, var_table)?;
            (
                (
                    (
                        TExpr::BuiltinFnCall(Box::new(call)),
                        asa[0].info.span.clone(),
                    ),
                    ttype,
                ),
                0,
            )
        },

        // negative & positive & not
        EO::Neg => {
            // wrap the sub-expr that this negates
            let (expr, idx) = wrap_expr(&asa[1..], type_table, var_table)?;

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
            let (expr, idx) = wrap_expr(&asa[1..], type_table, var_table)?;

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
        EO::Not => {
            // wrap the sub-expr that this nots
            let (expr, idx) = wrap_expr(&asa[1..], type_table, var_table)?;

            // make sure it's a number, otherwise throw error
            if expr.1 != Type::Bool {
                return Err(Error::NotBoolean {
                    oper_span: asa[0].info.span.clone(),
                    value_span: expr.0.1,
                    value_type: expr.1,
                });
            }

            // return notted value
            let expr_span = expr.0.1.clone();
            (
                (
                    (
                        TExpr::Not(Box::new(expr)), // value
                        asa[0].info.span.start..expr_span.end, // span
                    ),
                    Type::Bool, // type
                ),
                idx + 1, // idx (+1 as that's the offset given to the wrapper that produced it)
            )
        },

        // arithmatic / maths
        EO::Add => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

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
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

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
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

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
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

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
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // make sure lhs is of type string, otherwise throw error
            if lhs.1 != Type::String {
                return Err(Error::ConcatNonString {
                    oper_span: asa[0].info.span.clone(),
                    value_span: lhs.0.1.clone(),
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type string, otherwise throw error
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
                        TExpr::Concat(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::String, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },

        // getting variables
        EO::Ident(ident) => {
            // try get the type of the variable from the var-table, otherwise throw error
            let (ident, var_type) = match var_table.get(ident) {
                Some((ident, entry)) => (ident, entry.var_type.clone()),
                None => return Err(Error::VarNotFound { span: asa[0].info.span.clone() })
            };

            (
                (
                    (
                        TExpr::VarGet { ident, var_type: var_type.clone() },
                        asa[0].info.span.clone(),
                    ),
                    var_type,
                ),
                0,
            )
        },

        EO::EE => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // get the type of lhs and make sure it's the same as the rhs
            if lhs.1 != rhs.1 {
                return Err(Error::OperationTypeMismatch {
                    lhs_span: lhs.0.1,
                    lhs_type: lhs.1,
                    oper_span: asa[0].info.span.clone(),
                    rhs_span: rhs.0.1,
                    rhs_type: rhs.1,
                });
            }

            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::EE(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::NE => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // get the type of lhs and make sure it's the same as the rhs
            if lhs.1 != rhs.1 {
                return Err(Error::OperationTypeMismatch {
                    lhs_span: lhs.0.1,
                    lhs_type: lhs.1,
                    oper_span: asa[0].info.span.clone(),
                    rhs_span: rhs.0.1,
                    rhs_type: rhs.1,
                });
            }

            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::NE(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::GT => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // get the type of lhs and make sure it's the same as the rhs
            if lhs.1 != rhs.1 {
                return Err(Error::OperationTypeMismatch {
                    lhs_span: lhs.0.1,
                    lhs_type: lhs.1,
                    oper_span: asa[0].info.span.clone(),
                    rhs_span: rhs.0.1,
                    rhs_type: rhs.1,
                });
            }

            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::GT(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::LT => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // get the type of lhs and make sure it's the same as the rhs
            if lhs.1 != rhs.1 {
                return Err(Error::OperationTypeMismatch {
                    lhs_span: lhs.0.1,
                    lhs_type: lhs.1,
                    oper_span: asa[0].info.span.clone(),
                    rhs_span: rhs.0.1,
                    rhs_type: rhs.1,
                });
            }

            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::LT(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::GTE => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // get the type of lhs and make sure it's the same as the rhs
            if lhs.1 != rhs.1 {
                return Err(Error::OperationTypeMismatch {
                    lhs_span: lhs.0.1,
                    lhs_type: lhs.1,
                    oper_span: asa[0].info.span.clone(),
                    rhs_span: rhs.0.1,
                    rhs_type: rhs.1,
                });
            }

            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::GTE(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::LTE => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // get the type of lhs and make sure it's the same as the rhs
            if lhs.1 != rhs.1 {
                return Err(Error::OperationTypeMismatch {
                    lhs_span: lhs.0.1,
                    lhs_type: lhs.1,
                    oper_span: asa[0].info.span.clone(),
                    rhs_span: rhs.0.1,
                    rhs_type: rhs.1,
                });
            }

            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::LTE(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::And => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // make sure lhs is of type bool, otherwise throw error
            if lhs.1 != Type::Bool {
                return Err(Error::NotBoolean {
                    oper_span: asa[0].info.span.clone(),
                    value_span: lhs.0.1,
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type bool, otherwise throw error
            if rhs.1 != Type::Bool {
                return Err(Error::NotBoolean {
                    oper_span: asa[0].info.span.clone(),
                    value_span: rhs.0.1,
                    value_type: rhs.1,
                });
            }

            // return typed and operation
            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::And(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },
        EO::Or => {
            // wrap the left-hand side of this operation
            let (lhs, idx) = wrap_expr(&asa[1..], type_table, var_table)?;
            // wrap the rigth-hand side of this operation
            let (rhs, idx1) = wrap_expr(&asa[idx+2..], type_table, var_table)?;

            // make sure lhs is of type bool, otherwise throw error
            if lhs.1 != Type::Bool {
                return Err(Error::NotBoolean {
                    oper_span: asa[0].info.span.clone(),
                    value_span: lhs.0.1,
                    value_type: lhs.1,
                });
            }

            // make sure rhs is of type bool, otherwise throw error
            if rhs.1 != Type::Bool {
                return Err(Error::NotBoolean {
                    oper_span: asa[0].info.span.clone(),
                    value_span: rhs.0.1,
                    value_type: rhs.1,
                });
            }

            // return typed or operation
            let span = lhs.0.1.start..rhs.0.1.end;
            (
                (
                    (
                        TExpr::Or(Box::new(lhs), Box::new(rhs)), // value
                        span, // span
                    ),
                    Type::Bool, // type
                ),
                idx1 + idx + 2, // the current idx (accounting for offsets)
            )
        },

        _ => todo!(),
    })
}
