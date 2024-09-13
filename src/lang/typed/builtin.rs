use ketchup::Span;
use crate::lang::{error::typed::Error, parser::expr::Expr, typed::{expr::wrap_expr, symbol_table::TypeTable, types::Type}, Spanned};
use super::{expr::TExpr, types::Typed};

/// A tree representation of a builtin-function call
#[derive(Debug)]
pub enum TBuiltinFnCall {
    PrintLn(Option<Typed<Spanned<TExpr>>>),
}

/// Add type annotations to builtin-function calls
pub fn wrap_builtin(ident: &str, ident_span: Span, span: Span, args: &[Expr], _type_table: &TypeTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    match ident {
        "println" => builtin_println(span, args, _type_table),

        // if the builtin function is not found, then return error
        _ => return Err(Error::BuiltinNotFound {
            ident_span,
            ident: ident.to_string(),
            call_span: span,
        })
    }
}

/// Add type annotations to `println` builtin-function calls
fn builtin_println(span: Span, args: &[Expr], _type_table: &TypeTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's only one or no arguments, otherwise, throw error
    if args.len() > 1 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 0..2,
            arg_span: args[1].span.clone(),
        })
    }

    // if there are no arugments, return early
    if args.is_empty() {
        return Ok((
            TBuiltinFnCall::PrintLn(None),
            Type::Nil,
        ));
    }
    
    // evaluate the first and *only* argument and make sure it's a string, otherwise throw error
    let (arg, _) = wrap_expr(&args[0].asa, _type_table)?;
    if arg.1 != Type::String {
        return Err(
            Error::BuiltinWrongType {
                call_span: span,
                expected: Type::String,
                arg_type: arg.1,
                arg_span: arg.0.1,
            }
        );
    }

    // return println call
    Ok((
        TBuiltinFnCall::PrintLn(Some(arg)),
        Type::Nil,
    ))
}
