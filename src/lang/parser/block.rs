use ketchup::{error::KError, Span};
use logos::SpannedIter;
use crate::lang::{error::Error, token::Token};
use super::stmt::{parse_stmt, Stmt};

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<(Stmt, Span)>,
    pub tail: Option<(Stmt, Span)>,
}

/// Parses a block (given that the `LBrace` token as alredy been consumed)
pub fn parse_block(tokens: &mut SpannedIter<'_, Token>) -> Result<(Block, Span), Vec<KError<Token, Error>>> {
    let start_span = tokens.span();
    let mut stmts = Vec::new();

    // get the first token of the block and also check for `}` for emtpy blocks
    let first_tok = match tokens.next() {
        Some((Ok(Token::RBrace), span)) => return Ok((Block { stmts, tail: None }, start_span.start..span.end)),
        Some((Ok(token), span)) => Some((Ok(token), span)), // return token
        Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
        None => return Err(vec![KError::Other(tokens.span(), Error::UnclosedBrace)]),
    };

    // parse first statement
    let (stmt, mut next_tok) = parse_stmt(first_tok, tokens)?;
    stmts.push(stmt);

    while let Some((token, span)) = next_tok {
        match token {
            // only runs when there is a tail return
            Ok(Token::RBrace) => return Ok((Block { tail: stmts.pop(), stmts }, start_span.start..span.end)),
            
            Ok(Token::SemiColon) => {
                match tokens.next() {
                    // no tail return
                    Some((Ok(Token::RBrace), span)) => return Ok((Block { stmts, tail: None }, start_span.start..span.end)),
                    Some((Err(err), span)) => return Err(vec![KError::Other(span, err)]),
                    None => return Err(vec![KError::Other(tokens.span(), Error::ExpectedSemiOrRBrace)]),

                    token => {
                        let (stmt, local_next_tok) = parse_stmt(token, tokens)?; // parse next statement
                        stmts.push(stmt); // push statement to the stmt list
                        next_tok = local_next_tok; // update the next token
                    },
                }
            }

            Err(err) => return Err(vec![KError::Other(span, err)]),
            _ => return Err(vec![KError::Other(span, Error::ExpectedSemiOrRBrace)]),
        }
    }

    // this section of code can only be reached when the block is never terminated
    Err(vec![KError::Other(tokens.span(), Error::UnclosedBrace)])
}
