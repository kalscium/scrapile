use crate::lang::Spanned;
use super::types::Type;

/// A function type signature
#[derive(Debug, Clone)]
pub struct FuncSignature {
    pub params: Vec<Spanned<Type>>,
    pub retrn_type: Spanned<Type>,
}
