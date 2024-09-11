#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Number,
    String,

    Custom {
        ident: String,
        properties: Vec<Type>,
    },
}

pub type Typed<T> = (T, Type);
