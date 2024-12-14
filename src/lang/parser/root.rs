//! The root of the file (is never placed within a block or scope)

use ketchup::error::KError;
use logos::SpannedIter;
use crate::lang::{error::parser::Error, parser::block::parse_block, token::Token, Spanned};
use super::block::Block;

#[derive(Debug, Default)]
pub struct Roots {
    pub main: Vec<Spanned<Block>>,
}

/// Parses the roots of the project (stuff that will never be placed in a within any block or scope)
pub fn parse_root(tokens: &mut SpannedIter<'_, Token>) -> Result<Roots, Vec<KError<Error>>> {
    let mut roots = Roots::default();
    
    // parse every root in the project
    while let Some((token, span)) = tokens.next() {
        match token {
            Err(err) => return Err(vec![KError::Other(span, err)]),

            Ok(Token::Main) => {
                // parse and push the main root
                let (main, span) = parse_main(tokens)?;
                roots.main.push((main, span));
            },
            
            _ => return Err(vec![KError::Other(span, Error::ExpectedRoot)]),
        }
    }

    Ok(roots)
}

/// Parses the main body of the program (assuming the main keyword was already consumed)
pub fn parse_main(tokens: &mut SpannedIter<'_, Token>) -> Result<Spanned<Block>, Vec<KError<Error>>> {
    let start_span = tokens.span();
    
    // ensure that there is a `LBrace`
    match tokens.next() {
        Some((Ok(Token::LBrace), _)) => (),
        _ => return Err(vec![KError::Other(tokens.span(), Error::ExpectedBlockForMain { ctx_span: start_span })]),
    }
    
    let block = parse_block(tokens)?; // parse body
    Ok(block)
}
