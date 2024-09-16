use std::fmt::Display;
use crate::lang::{error::typed::Error, Spanned};
use super::symbol_table::TypeTable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Bool,
    Nil,

    Tuple(Vec<Type>),
    List(Box<Type>),

    Custom {
        ident: String,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Type::Number => "num".to_string(),
            Type::Nil => "nil".to_string(),
            Type::String => "str".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Tuple(types) => format!("({})", types.into_iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ")), // may not be the most efficient
            Type::List(list_type) => format!("[{}]", list_type),
            Type::Custom { ident } => format!("struct {{{ident}}}"),
        })
    }
}

/// Verifies if a type exists or not and returns it again
pub fn verify(atype: Spanned<Type>, type_table: &TypeTable) -> Result<Type, Error> {
    // only need to check the validity of custom types
    if let (Type::Custom { ref ident }, span) = atype {
        // make sure it exists in the type table, otherwise throw error
        if type_table.0.get(ident).is_none() {
            return Err(Error::TypeNotFound { span });
        }
    }

    Ok(atype.0)
}

pub type Typed<T> = (T, Type);
