use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,

    Custom {
        ident: String,
        properties: Vec<Type>,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Type::Number => "num",
            Type::String => "str",
            Type::Custom { .. } => todo!(),
        })
    }
}

pub type Typed<T> = (T, Type);
