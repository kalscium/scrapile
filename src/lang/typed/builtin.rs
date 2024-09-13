use ketchup::Span;
use crate::lang::{error::typed::Error, parser::expr::Expr, typed::{expr::wrap_expr, symbol_table::TypeTable, types::Type}, Spanned};
use super::{expr::TExpr, types::Typed};

/// A tree representation of a builtin-function call
#[derive(Debug)]
pub enum TBuiltinFnCall {
    PrintLn(Option<Spanned<TExpr>>),
    AsString(Spanned<TExpr>),
    Input(Spanned<TExpr>),
}

/// Add type annotations to builtin-function calls
pub fn wrap_builtin(ident: &str, ident_span: Span, span: Span, args: &[Expr], type_table: &TypeTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    match ident {
        "println" => builtin_println(span, args, type_table),
        "as_str" => builtin_as_str(span, args, type_table),
        "input" => builtin_input(span, args, type_table),

        // if the builtin function is not found, then return error
        _ => return Err(Error::BuiltinNotFound {
            ident_span,
            ident: ident.to_string(),
            call_span: span,
        })
    }
}

/// Add type annotations to `as_str` builtin-function calls
fn builtin_as_str(span: Span, args: &[Expr], type_table: &TypeTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's at least one argument
    if args.len() < 1 {
        return Err(Error::BuiltinLittleArgs {
            call_span: span,
            min: 1..2,
        });
    }

    // make sure there's only one argument
    if args.len() > 1 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 0..2,
            arg_span: args[1].span.clone(),
        });
    }

    // evaulate the argument and return it as a string
    let ((arg, _), _) = wrap_expr(&args[0].asa, type_table)?;
    Ok((
        TBuiltinFnCall::AsString(arg),
        Type::String,
    ))
}

/// Add type annotations to `input` builtin-function calls
fn builtin_input(span: Span, args: &[Expr], type_table: &TypeTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's at least one argument
    if args.len() < 1 {
        return Err(Error::BuiltinLittleArgs {
            call_span: span,
            min: 1..2,
        });
    }

    // make sure there's only one argument
    if args.len() > 1 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 0..2,
            arg_span: args[1].span.clone(),
        });
    }

    // evaulate the argument and return the `input` call
    let ((arg, _), _) = wrap_expr(&args[0].asa, type_table)?;
    Ok((
        TBuiltinFnCall::Input(arg),
        Type::String,
    ))
}

/// Add type annotations to `println` builtin-function calls
fn builtin_println(span: Span, args: &[Expr], type_table: &TypeTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's only one or no arguments, otherwise, throw error
    if args.len() > 1 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 0..2,
            arg_span: args[1].span.clone(),
        });
    }

    // if there are no arugments, return early
    if args.is_empty() {
        return Ok((
            TBuiltinFnCall::PrintLn(None),
            Type::Nil,
        ));
    }
    
    // evaluate the first and *only* argument and make sure it's a string, otherwise throw error
    let (arg, _) = wrap_expr(&args[0].asa, type_table)?;
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
        TBuiltinFnCall::PrintLn(Some(arg.0)),
        Type::Nil,
    ))
}
