use json::JsonValue::{self, Null};
use crate::scratch::expr_idx_to_id;
use super::{parse_stmt, Statement};

/// Adds a block of statements and returns the id of the first statement
pub(super) fn parse_block(block: Vec<Statement>, expr_blocks: &mut Vec<JsonValue>) -> String {
    let stmt_blocks = block.into_iter()
        .map(|stmt| parse_stmt(stmt, expr_blocks))
        .collect::<Vec<_>>();

    let id = expr_idx_to_id(expr_blocks.len()); // the next expr block will be the start of the block

    // insert the statement blocks
    let stmt_blocks_len = stmt_blocks.len();
    for (i, mut stmt_block) in stmt_blocks.into_iter().enumerate() {
        let idx = expr_blocks.len(); // true block index with expr blocks as an offset

        // update the link to the next block (if there is one)
        if i == stmt_blocks_len-1 { // last block shouldn't have a 'next' field
            stmt_block["next"] = Null;
        } else {
            stmt_block["next"] = expr_idx_to_id(idx+1).into();
        }

        // set other builderplate fields
        stmt_block["shadow"] = false.into();
        stmt_block["topLevel"] = false.into();
        stmt_block["parent"] = Null;

        expr_blocks.push(stmt_block);
    }

    id
}
