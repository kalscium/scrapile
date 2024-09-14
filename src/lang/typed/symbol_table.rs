use std::collections::HashMap;
use ketchup::Span;
use crate::lang::Spanned;
use super::types::Type;

/// A hashmap of all the types in a project, the key is the identifier of the type, each type is another hashmap of string property identifiers corresponding to a `u32` unique identifier and a type for that property
#[derive(Debug)]
pub struct TypeTable(
    pub HashMap<String, HashMap<String, (u32, Type)>>,
);

/// A hashmap of all the variables in a given scope
#[derive(Debug)]
pub struct VarTable {
    /// The prefix given to the final names of the variables it stores
    pub prefix: String,
    /// The acutal variable table
    pub table: HashMap<String, VarTableEntry>,

    /// A scope counter
    scopes: usize,
}

/// An entry in the VarTable
#[derive(Debug)]
pub struct VarTableEntry {
    /// The type of the variable
    pub var_type: Spanned<Type>,
    /// If the variable is mutable
    pub mutable: bool,
    /// Span of the variable
    pub span: Span,
}

impl VarTable {
    /// Creates a new emtpy var-table
    #[inline]
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            table: HashMap::new(),
            scopes: 0,
        }
    }
    
    /// Spawns an new child variable table with a custom prefix
    pub fn spawn(&self, prefix: &str) -> Self {
        Self {
            prefix: format!("{}{prefix}", self.prefix),
            table: HashMap::new(),
            scopes: 0,
        }
    }

    /// Spawns a new child variable table with without a custom prefix
    pub fn spawn_scope(&mut self) -> Self {
        self.scopes += 1; // increase the scope counter
        Self {
            prefix: format!("{}${}", self.prefix, self.scopes),
            table: HashMap::new(),
            scopes: 0,
        }
    }
}
