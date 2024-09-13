use ketchup::Span;
use crate::lang::{error::typed::Error, parser::expr::Expr, Spanned};
use super::{expr::TExpr, types::Typed};

#[derive(Debug)]
pub enum TBuiltinFnCall {
    PrintLn(Typed<Spanned<TExpr>>),
}

pub fn wrap_builtin(ident: &str, ident_span: Span, span: Span, args: &[Expr]) -> Result<Typed<TBuiltinFnCall>, Error> {
    match ident {
        "println" => todo!(),

        // if the builtin function is not found, then return error
        _ => return Err(Error::BuiltinNotFound {
            ident_span: ident_span,
            ident: ident.to_string(),
            call_span: span,
        })
    }
}
