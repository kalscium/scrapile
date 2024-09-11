use ketchup::node::Node;
use crate::lang::{parser::{block::Block, expr::ExprOper}, typed::types::Type};
use super::{symbol_table::TypeTable, types::Typed};

/// A tree version of expr for type annotation
#[derive(Debug)]
pub enum TExpr {
    Number(f64),
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
pub fn wrap_expr(asa: &[Node<ExprOper>], _symbol_table: &TypeTable) -> Typed<TExpr> {
    use ExprOper as EO;

    match &asa[0].oper {
        // literals
        EO::Number(num) => (TExpr::Number(*num), Type::Number),
        EO::String(string) => (TExpr::String(string.clone()), Type::String),

        _ => todo!(),
    }
}
