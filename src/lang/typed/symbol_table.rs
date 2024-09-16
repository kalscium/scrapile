use std::collections::HashMap;
use ketchup::Span;
use super::types::Type;

/// A hashmap of all the types in a project, the key is the identifier of the type, each type is another hashmap of string property identifiers corresponding to a `u32` unique identifier and a type for that property
#[derive(Debug)]
pub struct TypeTable(
    pub HashMap<String, HashMap<String, (u32, Type)>>,
);

/// A hashmap of all the variables in a given scope
#[derive(Debug, Clone)]
pub struct VarTable {
    /// The prefix given to the final names of the variables it stores
    prefix: String,

    /// An optional parent table / scope
    parent: Option<Box<VarTable>>,

    /// The acutal variable table
    table: HashMap<String, VarTableEntry>,

    /// A scope counter
    scopes: usize,
}

/// An entry in the VarTable
#[derive(Debug, Clone)]
pub struct VarTableEntry {
    /// The type of the variable
    pub var_type: Type,
    /// If the variable is mutable
    pub mutable: bool,
    /// Span of the variable
    pub span: Span,
}

impl VarTable {
    // # Variable Identifier Naming Scheme
    // ---
    // $: a compiler generated name for a scope
    // #: user defined functions
    // /: user defined variables (within scopes)

    /// Creates a new emtpy var-table
    #[inline]
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            parent: None,
            table: HashMap::new(),
            scopes: 0,
        }
    }

    /// Returns a unique identifier according to the current scope
    #[inline]
    pub fn get_ident(&self, ident: &str) -> String {
        format!("{}/{ident}", self.prefix)
    }

    /// Gets the a variable entry and it's identifier from either this scope or it's parent scope
    pub fn get(&self, key: &str) -> Option<(String, &VarTableEntry)> {
        match self.table.get(key) {
            Some(entry) => Some((self.get_ident(key), entry)),
            None => {
                match &self.parent {
                    Some(parent) => parent.get(key),
                    None => None,
                }
            },
        }
    }

    /// Inserts a variable entry into the current scope
    #[inline]
    pub fn insert(&mut self, key: String, entry: VarTableEntry) {
        self.table.insert(key, entry);
    }
    
    /// Spawns an new child variable table with a custom prefix
    pub fn spawn(&self, prefix: &str) -> Self {
        Self {
            prefix: format!("{}{prefix}", self.prefix),
            parent: Some(Box::new(self.clone())),
            table: HashMap::new(),
            scopes: 0,
        }
    }

    /// Spawns a new child variable table with without a custom prefix
    pub fn spawn_scope(&mut self) -> Self {
        self.scopes += 1; // increase the scope counter
        Self {
            prefix: format!("{}${}", self.prefix, self.scopes),
            parent: Some(Box::new(self.clone())),
            table: HashMap::new(),
            scopes: 0,
        }
    }
}
