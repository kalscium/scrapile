pub mod error;
pub mod token;
pub mod parser;
pub mod typed;

pub type Spanned<T> = (T, ketchup::Span);
