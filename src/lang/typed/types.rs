use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number,
    String,
    Bool,
    Nil,

    Tuple(Vec<Type>),

    Custom {
        ident: String,
        properties: Vec<Type>,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Type::Number => "num".to_string(),
            Type::Nil => "nil".to_string(),
            Type::String => "str".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Tuple(types) => format!("tuple({})", types.into_iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ")), // may not be the most efficient
            Type::Custom { .. } => todo!(),
        })
    }
}

pub type Typed<T> = (T, Type);
