use crate::lang::{error::typed::Error, parser::stmt::Stmt, typed::{symbol_table::VarTableEntry, types}, Spanned};
use super::{expr::{wrap_expr, TExpr}, symbol_table::{FuncTable, TypeTable, VarTable}, types::{Type, Typed}};

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

    /// A variable mutation with `mut`
    VarMutate {
        ident: String,
        value: Typed<Spanned<TExpr>>,
    },

    /// An if statement
    If {
        cond: Typed<Spanned<TExpr>>,
        body: Box<Typed<TStmt>>,
        otherwise: Option<Box<Typed<TStmt>>>,
    },

    /// A while statement
    While {
        cond: Typed<Spanned<TExpr>>,
        body: Box<Typed<TStmt>>,
    },
}

/// Adds type annotations to a statement
pub fn wrap_stmt(stmt: Spanned<Stmt>, type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TStmt>, Error> {
    let (stmt, span) = stmt;
    match stmt {
        Stmt::Expr(expr) => {
            let expr = wrap_expr(&expr.asa, type_table, func_table, var_table)?.0;
            let stmt_type = expr.1.clone();

            Ok((TStmt::Expr(expr.0.0), stmt_type))
        },
        Stmt::VarMutate { ident, value } => {
            let (value, _) = wrap_expr(&value.asa, type_table, func_table, var_table)?; // wrap value

            // make sure the variable exists and if so, get the type of it
            let (var_ident, var_type, mutable, var_span) = match var_table.get(&ident.0) {
                Some((ident, entry)) => (ident, entry.var_type.clone(), entry.mutable, entry.span.clone()),
                None => return Err(Error::VarNotFound { span: ident.1 }),
            };

            // make sure the variable is mutable in the first place
            if !mutable {
                return Err(Error::AssignToImmutable { var_span, span });
            }
            
            // make sure the type of the variable and the type of the value are the same, otherwise throw error
            if value.1 != var_type {
                return Err(Error::VarTypeMismatch { span: value.0.1, type_span: var_span, expr_type: value.1, var_type });
            }

            // return valid variable mutation
            Ok((
                TStmt::VarMutate { ident: var_ident, value },
                Type::Nil
            ))
        },
        Stmt::VarDeclare { mutable, ident, atype, value } => {
            let (value, _) = wrap_expr(&value.asa, type_table, func_table, var_table)?; // wrap value

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
        Stmt::If { cond, body, otherwise } => {
            // wrap the condition
            let (cond, _) = wrap_expr(&cond.asa, type_table, func_table, var_table)?;

            // make sure the condition is a boolean
            if cond.1 != Type::Bool {
                return Err(Error::NonBoolCond { span: cond.0.1, expr_type: cond.1, ctx_span: span });
            }

            // wrap the body and ifelse
            let body = wrap_stmt(*body, type_table, func_table, var_table)?;
            let ifelse = match otherwise {
                Some(ifelse) => Some(Box::new(wrap_stmt(*ifelse, type_table, func_table, var_table)?)),
                None => None,
            };

            // return valid if statement
            Ok((
                TStmt::If { cond, body: Box::new(body), otherwise: ifelse },
                Type::Nil,
            ))
        },
        Stmt::While { cond, body } => {
            // wrap the condition
            let (cond, _) = wrap_expr(&cond.asa, type_table, func_table, var_table)?;

            // make sure the condition is a boolean
            if cond.1 != Type::Bool {
                return Err(Error::NonBoolCond { span: cond.0.1, expr_type: cond.1, ctx_span: span });
            }

            // wrap the body
            let body = wrap_stmt(*body, type_table, func_table, var_table)?;

            // return valid while statement
            Ok((
                TStmt::While { cond, body: Box::new(body) },
                Type::Nil,
            ))
        },
    }
}
