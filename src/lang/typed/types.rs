#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Float,
    UInt,
    UFloat,
    Int,
    String,

    /// A reference to another value of a certain type
    Ref(Box<Type>),
    /// A mutable reference to another value of a certain type
    MutRef(Box<Type>),

    Custom {
        ident: String,
        properties: Vec<Type>,
    },
}

pub type Typed<T> = (T, Type);
