use crate::lang::{error::typed::Error, parser::block::Block, typed::{expr, types::Type}, Spanned};
use super::{expr::TExpr, symbol_table::TypeTable, types::Typed};

#[derive(Debug)]
pub struct TBlock {
    pub stmts: Vec<Typed<Spanned<TExpr>>>,
    pub tail: Option<Typed<Spanned<TExpr>>>,
}

/// Adds type annotations to a block
pub fn wrap_block(block: Block, _type_table: &TypeTable) -> Result<Typed<TBlock>, Error> {
    // iterate through the block's statements and add type annotations to all of them
    let mut stmts = Vec::new();
    for (stmt, _) in block.stmts {
        stmts.push(
            expr::wrap_expr(&stmt.0.asa, _type_table)?.0
        );
    }

    // check the type of the tail, (if there is one)
    let (tail, tail_type) = match block.tail {
        None => (None, Type::Nil),
        Some((stmt, _)) => {
            let (stmt, _) = expr::wrap_expr(&stmt.0.asa, _type_table)?;
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
