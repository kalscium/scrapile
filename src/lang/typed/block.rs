use crate::lang::{error::typed::Error, parser::block::Block, typed::{stmt, types::Type}, Spanned};
use super::{stmt::TStmt, symbol_table::{TypeTable, VarTable}, types::Typed};

/// A tree representation of a block
#[derive(Debug)]
pub struct TBlock {
    pub stmts: Vec<Typed<Spanned<TStmt>>>,
    pub tail: Option<Typed<Spanned<TStmt>>>,
}

/// Adds type annotations to a block
pub fn wrap_block(block: Block, type_table: &TypeTable, mut var_table: VarTable) -> Result<Typed<TBlock>, Error> {
    // iterate through the block's statements and add type annotations to all of them
    let mut stmts = Vec::new();
    for (stmt, span) in block.stmts {
        let (stmt, stmt_type) = stmt::wrap_stmt((stmt, span.clone()), type_table, &mut var_table)?;
        stmts.push(((stmt, span), stmt_type));
    }

    // check the type of the tail, (if there is one)
    let (tail, tail_type) = match block.tail {
        None => (None, Type::Nil),
        Some((stmt, span)) => {
            let (stmt, stmt_type) = stmt::wrap_stmt((stmt, span.clone()), type_table, &mut var_table)?;

            (Some(((stmt, span), stmt_type.clone())), stmt_type)
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
