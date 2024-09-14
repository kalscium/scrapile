use crate::lang::{error::typed::Error, parser::{block::Block, stmt::Stmt}, typed::{expr, types::Type}, Spanned};
use super::{expr::TExpr, symbol_table::TypeTable, types::Typed};

/// A tree representation of a block
#[derive(Debug)]
pub struct TBlock {
    pub stmts: Vec<Typed<Spanned<TExpr>>>,
    pub tail: Option<Typed<Spanned<TExpr>>>,
}

/// Adds type annotations to a block
pub fn wrap_block(block: Block, type_table: &TypeTable) -> Result<Typed<TBlock>, Error> {
    // iterate through the block's statements and add type annotations to all of them
    let mut stmts = Vec::new();
    for (stmt, _) in block.stmts {
        stmts.push(match &stmt {
            Stmt::Expr(expr) => expr::wrap_expr(&expr.asa, type_table)?.0,
        });
    }

    // check the type of the tail, (if there is one)
    let (tail, tail_type) = match block.tail {
        None => (None, Type::Nil),
        Some((stmt, _)) => {
            let (stmt, _) = match &stmt {
                Stmt::Expr(expr) => expr::wrap_expr(&expr.asa, type_table)?,
            };
            let stmt_type = stmt.1.clone();

            (Some(stmt), stmt_type)
        },
    };

    // return type annotated block
    Ok((
        TBlock {
            stmts,
            tail,
        },
        tail_type,
    ))
}
