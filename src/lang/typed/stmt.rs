use crate::lang::{error::typed::Error, parser::stmt::Stmt};

use super::{expr::{wrap_expr, TExpr}, symbol_table::TypeTable, types::Typed};

/// A tree version of a stmt for type annotations
#[derive(Debug)]
pub enum TStmt {
    Expr(TExpr),
}

/// Adds type annotations to a statement
pub fn wrap_stmt(stmt: Stmt, type_table: &TypeTable) -> Result<Typed<TStmt>, Error> {
    match stmt {
        Stmt::Expr(expr) => {
            let expr = wrap_expr(&expr.asa, type_table)?.0;
            let stmt_type = expr.1.clone();

            Ok((TStmt::Expr(expr.0.0), stmt_type))
        },
        _ => todo!()
    }
}
