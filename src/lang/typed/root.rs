use std::collections::HashMap;

use crate::lang::{error::typed::Error, parser::root::Root, Spanned};
use super::{block::{self, TBlock}, symbol_table::TypeTable};

/// A type annotated representation of the entire project with all the roots evaluated statically
#[derive(Debug)]
pub struct Project {
    /// The main block / procedure
    pub main: Spanned<TBlock>,
    
    // /// Additional user-defined procedures
    // pub procedures: Vec<Spanned<TBlock>>,
}

/// Wraps the root of the project in types and returns a single, safe and valid project root
pub fn wrap_root(roots: &[Spanned<Root>]) -> Result<Project, Error> {
    let type_table = TypeTable(HashMap::new());
    let mut main = None;
    
    // annotate all the roots
    for (root, span) in roots {
        match root {
            // set the main block
            Root::Main(block) => {
                let (block, _) = block::wrap_block(block.clone(), &type_table)?; // wrap main block in types

                // try to set the main block, if there's already one, then throw an error
                if let Some((_, first_span)) = main.replace((block, span.clone())) {
                    return Err(Error::MultipleMain {
                        first_span,
                        additional_span: span.clone(),
                    });
                }
            }
        }
    }

    // unwrap the main procedure, if there isn't one, throw an error
    let main = match main {
        Some(main) => main,
        None => return Err(Error::NoMain),
    };

    Ok(Project {
        main,
    })
}
