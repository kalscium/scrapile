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

/// Wraps an expr with types
pub fn wrap_expr(asa: &[Node<ExprOper>], _symbol_table: &TypeTable) -> Result<Typed<Spanned<TExpr>>, Error> {
    use ExprOper as EO;

    Ok(match &asa[0].oper {
        // literals
        EO::Number(num) => ((TExpr::Number(*num), asa[0].info.span.clone()), Type::Number),
        EO::String(string) => ((TExpr::String(string.clone()), asa[0].info.span.clone()), Type::String),

        // negative & positive
        EO::Neg => {
            // wrap the sub-expr that this negates
            let expr = wrap_expr(&asa[1..], _symbol_table)?;

            // make sure it's a number, otherwise throw error
            if expr.1 != Type::Number {
                return Err(Error::CanOnlyNegNumber {
                    oper_span: asa[0].info.span.clone(),
                    value_span: expr.0.1,
                    value_type: expr.1,
                });
            }

            // return negated value
            let expr_span = expr.0.1.clone();
            ((TExpr::Neg(Box::new(expr)), asa[0].info.span.start..expr_span.end), Type::Number)
        },
        EO::Pos => {
            // wrap the sub-expr that this does nothing to
            let expr = wrap_expr(&asa[1..], _symbol_table)?;

            // make sure it's a number, otherwise throw error
            if expr.1 != Type::Number {
                return Err(Error::CanOnlyPosNumber {
                    oper_span: asa[0].info.span.clone(),
                    value_span: expr.0.1,
                    value_type: expr.1,
                });
            }

            // return negated value
            let expr_span = expr.0.1.clone();
            ((TExpr::Pos(Box::new(expr)), asa[0].info.span.start..expr_span.end), Type::Number)
        },

        _ => todo!(),
    })
}
