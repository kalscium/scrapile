use crate::lang::{error::typed::Error, parser::stmt::Stmt, typed::{symbol_table::VarTableEntry, types}, Spanned};
use super::{expr::{wrap_expr, TExpr}, symbol_table::{TypeTable, VarTable}, types::{Type, Typed}};

/// A tree version of a stmt for type annotations
#[derive(Debug)]
pub enum TStmt {
    /// An expression
    Expr(TExpr),

    /// A variable declaration with `let`
    VarDeclare {
        ident: String,
        value: Typed<Spanned<TExpr>>,
    },
}

/// Adds type annotations to a statement
pub fn wrap_stmt(stmt: Spanned<Stmt>, type_table: &TypeTable, var_table: &mut VarTable) -> Result<Typed<TStmt>, Error> {
    let (stmt, span) = stmt;
    match stmt {
        Stmt::Expr(expr) => {
            let expr = wrap_expr(&expr.asa, type_table, var_table)?.0;
            let stmt_type = expr.1.clone();

            Ok((TStmt::Expr(expr.0.0), stmt_type))
        },
        Stmt::VarDeclare { mutable, ident, atype, value } => {
            let (value, _) = wrap_expr(&value.asa, type_table, var_table)?; // wrap value

            // make sure the type annotations and type of the value are the same
            if let Some((atype, span)) = atype {
                let atype = types::verify((atype, span.clone()), type_table)?; // verify the type exists
                if value.1 != atype {
                    return Err(Error::VarTypeMismatch { span: value.0.1, type_span: span, expr_type: value.1, var_type: atype });
                }
            }

            // update variable table
            var_table.insert(ident.clone(), VarTableEntry {
                var_type: value.1.clone(),
                mutable,
                span,
            });
            
            // return completed variable declaration
            Ok((
                TStmt::VarDeclare { ident: var_table.get_ident(&ident), value },
                Type::Nil,
            ))
        },
    }
}
