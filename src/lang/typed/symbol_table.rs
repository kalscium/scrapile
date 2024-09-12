use std::collections::HashMap;
use super::types::Type;

/// A hashmap of all the types in a project, the key is the identifier of the type, each type is another hashmap of string property identifiers corresponding to a `u32` unique identifier and a type for that property
#[derive(Debug)]
pub struct TypeTable(
    pub HashMap<String, HashMap<String, (u32, Type)>>,
);
