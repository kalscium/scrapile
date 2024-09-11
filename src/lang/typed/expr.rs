use ketchup::node::Node;
use crate::lang::parser::{block::Block, expr::ExprOper};
use super::{symbol_table::TypeTable, types::Typed};

/// A tree version of expr for type annotation
#[derive(Debug)]
pub enum TExpr {
    Integer(u32),
    Float(f64),
    String(String),
    Ident(String),

    Add(Box<Typed<TExpr>>),
    Sub(Box<Typed<TExpr>>),
    Mul(Box<Typed<TExpr>>),
    Div(Box<Typed<TExpr>>),
    Concat(Box<Typed<TExpr>>),

    DotAccess(u32),

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

    Tuple(Vec<Typed<TExpr>>),
    Call(String, Vec<Typed<TExpr>>),
    BuiltinFnCall(String, Vec<Vec<Typed<TExpr>>>),
    Block(Typed<Block>),
}

/// Wraps an expr with types
pub fn wrap_expr(asa: &[Node<ExprOper>], symbol_table: &TypeTable) -> Typed<TExpr> {
    // wrap the exprs
    for node in asa {
        todo!()
    }

    todo!()
}
