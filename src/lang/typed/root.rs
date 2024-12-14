use std::collections::HashMap;

use crate::lang::{error::typed::Error, parser::root::Roots, Spanned};
use super::{block::{self, TBlock}, symbol_table::{TypeTable, VarTable}};

/// A type annotated representation of the entire project with all the roots evaluated statically
#[derive(Debug)]
pub struct Project {
    /// The main block / procedure
    pub main: Spanned<TBlock>,
    
    // /// Additional user-defined procedures
    // pub procedures: Vec<Spanned<TBlock>>,
}

/// Wraps the root of the project in types and returns a single, safe and valid project root
pub fn wrap_root(roots: &Roots) -> Result<Project, Error> {
    let type_table = TypeTable(HashMap::new());

    // make sure there's one and only one main root, otherwise throw an error
    let Some(main) = roots.main.get(0)
    else {
        return Err(Error::NoMain)        
    };
    if let Some(extra) = roots.main.get(1) {
        return Err(Error::MultipleMain {
            first_span: main.1.clone(),
            additional_span: extra.1.clone(),
        });
    }

    // wrap the main block in types
    let main = (
        block::wrap_block(main.0.clone(), &type_table, VarTable::new("$root".to_string()))?.0,
        main.1.clone(),
    );

    Ok(Project {
        main,
    })
}
