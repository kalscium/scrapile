use crate::lang::{error::typed::Error, parser::stmt::Stmt};
use super::{expr::{wrap_expr, TExpr}, symbol_table::{TypeTable, VarTable}, types::Typed};

/// A tree version of a stmt for type annotations
#[derive(Debug)]
pub enum TStmt {
    /// An expression
    Expr(TExpr),

    /// A variable declaration with `let`
    VarDeclare {
        ident: String,
        value: Typed<TExpr>,
    },
}

/// Adds type annotations to a statement
pub fn wrap_stmt(stmt: Stmt, type_table: &TypeTable, var_table: &mut VarTable) -> Result<Typed<TStmt>, Error> {
    match stmt {
        Stmt::Expr(expr) => {
            let expr = wrap_expr(&expr.asa, type_table, var_table)?.0;
            let stmt_type = expr.1.clone();

            Ok((TStmt::Expr(expr.0.0), stmt_type))
        },
        _ => todo!()
    }
}
