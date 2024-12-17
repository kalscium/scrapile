use std::collections::HashMap;
use crate::lang::{error::typed::Error, parser::root::Roots};
use super::{block::{self, TBlock}, function::FuncSignature, symbol_table::{FuncTable, TypeTable, VarTable, VarTableEntry}};

/// A type annotated representation of the entire project with all the roots evaluated statically
#[derive(Debug)]
pub struct Project {
    /// The main block / procedure
    pub main: TBlock,
    
    /// Additional user-defined procedures
    pub procedures: Vec<TBlock>,
}

/// Wraps the root of the project in types and returns a single, safe and valid project root
pub fn wrap_root(roots: &Roots) -> Result<Project, Error> {
    let type_table = TypeTable(HashMap::new());
    let mut func_table = FuncTable(HashMap::new());

    // iterate through the functions and gather their signatures
    for func in roots.funcs.iter() {
        let signature = (FuncSignature {
            params: func.0.params.iter()
                .map(|((_, ptype), span)| (ptype.clone(), span.clone()))
                .collect::<Vec<_>>(),
            retrn_type: func.0.retrn_type.clone(),
        }, func.1.clone());

        // insert and also check for duplicate function func.0initions
        if let Some(old) = func_table.0.insert(func.0.ident.clone(), signature) {
            return Err(Error::MultipleFunc {
                first_span: old.1,
                additional_span: func.1.clone(),
            })
        }
    }

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
    let main = block::wrap_block(
        main.0.clone(),
        &type_table,
        &func_table,
        VarTable::new("$root".to_string()),
    )?.0;

    // wrap the rest of the function definitions in types
    let mut procedures = Vec::new();
    for func in roots.funcs.iter() {
        let mut var_table = VarTable::new(format!("$func${}", func.0.ident));

        // insert the parameters
        for param in func.0.params.iter() {
            var_table.insert(param.0.0.clone(), VarTableEntry {
                var_type: param.0.1.clone(),
                mutable: false,
                span: param.1.clone(),
            });
        }

        // wrap the procedures
        let wrapped = block::wrap_block(func.0.body.0.clone(), &type_table, &func_table, var_table)?;

        // make sure the body's return value is of the right type
        if wrapped.1 != func.0.retrn_type.0 {
            return Err(Error::RetrnTypeMismatch {
                span: wrapped.0.tail.map(|((_, span), _)| span.clone()).unwrap_or_else(|| func.0.body.1.clone()),
                type_span: func.0.retrn_type.1.clone(),
                expr_type: wrapped.1.clone(),
                retrn_type: func.0.retrn_type.0.clone(),
            })
        }

        // push the wrapped block
        procedures.push(wrapped.0);
    }

    Ok(Project {
        main,
        procedures,
    })
}
