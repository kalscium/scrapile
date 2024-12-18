use ketchup::Span;
use crate::lang::{error::typed::Error, parser::expr::Expr, typed::{expr::wrap_expr, symbol_table::TypeTable, types::Type}, Spanned};
use super::{expr::TExpr, symbol_table::{FuncTable, VarTable}, types::Typed};

/// A tree representation of a builtin-function call
#[derive(Debug)]
pub enum TBuiltinFnCall {
    PrintLn(Option<Spanned<TExpr>>),
    AsString(Spanned<TExpr>),
    Input(Spanned<TExpr>),
    Timer,
    Panic(Span, Option<Spanned<TExpr>>),
    ListLen(Spanned<TExpr>),
    ListGet {
        span: Span,
        list: Spanned<TExpr>,
        idx: Spanned<TExpr>,
    },
    ListPush {
        list: Spanned<TExpr>,
        expr: Spanned<TExpr>,
    },
    ListInsert {
        span: Span,
        list: Spanned<TExpr>,
        idx: Spanned<TExpr>,
        expr: Spanned<TExpr>,
    },
    StringLen(Spanned<TExpr>),
    StringGet {
        span: Span,
        string: Spanned<TExpr>,
        idx: Spanned<TExpr>,
    },
}

/// Add type annotations to builtin-function calls
pub fn wrap_builtin(ident: &str, ident_span: Span, span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    match ident {
        "println" => builtin_println(span, args, type_table, func_table, var_table),
        "as_str" => builtin_as_str(span, args, type_table, func_table, var_table),
        "input" => builtin_input(span, args, type_table, func_table, var_table),
        "timer" => builtin_timer(span, args),
        "panic" => builtin_panic(span, args, type_table, func_table, var_table),
        "list_len" => builtin_list_len(span, args, type_table, func_table, var_table),
        "list_get" => builtin_list_get(span, args, type_table, func_table, var_table),
        "list_push" => builtin_list_push(span, args, type_table, func_table, var_table),
        "list_insert" => builtin_list_insert(span, args, type_table, func_table, var_table),
        "str_len" => builtin_str_len(span, args, type_table, func_table, var_table),
        "str_get" => builtin_str_get(span, args, type_table, func_table, var_table),

        // if the builtin function is not found, then return error
        _ => return Err(Error::BuiltinNotFound {
            ident_span,
            ident: ident.to_string(),
            call_span: span,
        })
    }
}

/// Add type annotations to `as_str` builtin-function calls
fn builtin_as_str(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
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
            max: 1..2,
            arg_span: args[1].span.clone(),
        });
    }

    // evaulate the argument and return it as a string
    let ((arg, _), _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;
    Ok((
        TBuiltinFnCall::AsString(arg),
        Type::String,
    ))
}

/// Add type annotations to `timer` builtin-function calls
fn builtin_timer(span: Span, args: &[Expr]) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's no arguments
    if args.len() != 0 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 0..1,
            arg_span: args[0].span.clone(),
        });
    }

    // return the `timer` call
    Ok((TBuiltinFnCall::Timer, Type::Number))
}

/// Add type annotations to `input` builtin-function calls
fn builtin_input(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
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
            max: 1..2,
            arg_span: args[1].span.clone(),
        });
    }

    // evaulate the argument and return the `input` call
    let ((arg, _), _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;
    Ok((
        TBuiltinFnCall::Input(arg),
        Type::String,
    ))
}

/// Add type annotations to `println` builtin-function calls
fn builtin_println(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
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
    let (arg, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;
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

/// Add type annotations to `println` builtin-function calls
fn builtin_panic(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
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
            TBuiltinFnCall::Panic(span, None),
            Type::Nil,
        ));
    }
    
    // evaluate the first and *only* argument and make sure it's a string, otherwise throw error
    let (arg, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;
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

    // return panic call
    Ok((
        TBuiltinFnCall::Panic(span, Some(arg.0)),
        Type::Nil,
    ))
}

/// Add type annotations to `list_len` builtin-function calls
fn builtin_list_len(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
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
            max: 1..2,
            arg_span: args[1].span.clone(),
        });
    }

    // wrap the expr and make sure it's of type list
    let (expr, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;
    match expr.1 {
        Type::List(_) => (),
        _ => return Err(Error::BuiltinArgTypeMismatch {
            span: expr.0.1,
            param_type: Type::List(Box::new(expr.1.clone())),
            arg_type: expr.1,
            call_span: span,
        }),
    }

    // return completed builtin-fn call
    Ok((
        TBuiltinFnCall::ListLen(expr.0),
        Type::Number,
    ))
}

/// Add type annotations to `list_get` builtin-function calls
fn builtin_list_get(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's at least two arguments
    if args.len() < 2 {
        return Err(Error::BuiltinLittleArgs {
            call_span: span,
            min: 2..3,
        });
    }

    // make sure there's only two arguments
    if args.len() > 2 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 2..3,
            arg_span: args[2].span.clone(),
        });
    }

    // wrap the list expr
    let (list_expr, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;

    // get the list type
    let Type::List(list_type) = list_expr.1
    else {
        return Err(Error::BuiltinArgTypeMismatch {
            span: list_expr.0.1,
            param_type: Type::List(Box::new(list_expr.1.clone())),
            arg_type: list_expr.1,
            call_span: span,
        });
    };

    // wrap the index expr and make sure it's of type nubmer
    let (idx_expr, _) = wrap_expr(&args[1].asa, type_table, func_table, var_table)?;
    match idx_expr.1 {
        Type::Number => (),
        _ => return Err(Error::BuiltinArgTypeMismatch {
            span: idx_expr.0.1,
            param_type: Type::Number,
            arg_type: idx_expr.1,
            call_span: span,
        }),
    }

    // return completed builtin-fn call
    Ok((
        TBuiltinFnCall::ListGet {
            span,
            list: list_expr.0,
            idx: idx_expr.0,
        },
        *list_type,
    ))
}

/// Add type annotations to `list_push` builtin-function calls
fn builtin_list_push(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's at least two arguments
    if args.len() < 2 {
        return Err(Error::BuiltinLittleArgs {
            call_span: span,
            min: 2..3,
        });
    }

    // make sure there's only two arguments
    if args.len() > 2 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 2..3,
            arg_span: args[2].span.clone(),
        });
    }

    // wrap the list expr
    let (list_expr, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;

    // get the list type
    let Type::List(list_type) = list_expr.1
    else {
        return Err(Error::BuiltinArgTypeMismatch {
            span: list_expr.0.1,
            param_type: Type::List(Box::new(list_expr.1.clone())),
            arg_type: list_expr.1,
            call_span: span,
        });
    };

    // wrap the element expr and make sure it's of the right type
    let (expr, _) = wrap_expr(&args[1].asa, type_table, func_table, var_table)?;
    if expr.1 != *list_type {
        return Err(Error::BuiltinArgTypeMismatch {
            span: expr.0.1,
            param_type: *list_type,
            arg_type: expr.1,
            call_span: span,
        });
    }

    // return completed builtin-fn call
    Ok((
        TBuiltinFnCall::ListPush {
            list: list_expr.0,
            expr: expr.0,
        },
        Type::Nil,
    ))
}

/// Add type annotations to `list_insert` builtin-function calls
fn builtin_list_insert(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's at least three arguments
    if args.len() < 3 {
        return Err(Error::BuiltinLittleArgs {
            call_span: span,
            min: 3..4,
        });
    }

    // make sure there's only three arguments
    if args.len() > 3 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 3..4,
            arg_span: args[3].span.clone(),
        });
    }

    // wrap the list expr
    let (list_expr, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;

    // get the list type
    let Type::List(list_type) = list_expr.1
    else {
        return Err(Error::BuiltinArgTypeMismatch {
            span: list_expr.0.1,
            param_type: Type::List(Box::new(list_expr.1.clone())),
            arg_type: list_expr.1,
            call_span: span,
        });
    };

    // wrap the index expr and make sure it's of type nubmer
    let (idx_expr, _) = wrap_expr(&args[1].asa, type_table, func_table, var_table)?;
    match idx_expr.1 {
        Type::Number => (),
        _ => return Err(Error::BuiltinArgTypeMismatch {
            span: idx_expr.0.1,
            param_type: Type::Number,
            arg_type: idx_expr.1,
            call_span: span,
        }),
    }

    // wrap the element expr and make sure it's of the right type
    let (expr, _) = wrap_expr(&args[2].asa, type_table, func_table, var_table)?;
    if expr.1 != *list_type {
        return Err(Error::BuiltinArgTypeMismatch {
            span: expr.0.1,
            param_type: *list_type,
            arg_type: expr.1,
            call_span: span,
        });
    }

    // return completed builtin-fn call
    Ok((
        TBuiltinFnCall::ListInsert {
            span,
            list: list_expr.0,
            idx: idx_expr.0,
            expr: expr.0,
        },
        Type::Nil,
    ))
}

/// Add type annotations to `str_len` builtin-function calls
fn builtin_str_len(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
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
            max: 1..2,
            arg_span: args[1].span.clone(),
        });
    }

    // wrap the expr and make sure it's of type str
    let (expr, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;
    match expr.1 {
        Type::String => (),
        _ => return Err(Error::BuiltinArgTypeMismatch {
            span: expr.0.1,
            param_type: Type::String,
            arg_type: expr.1,
            call_span: span,
        }),
    }

    // return completed builtin-fn call
    Ok((
        TBuiltinFnCall::StringLen(expr.0),
        Type::Number,
    ))
}

/// Add type annotations to `str_get` builtin-function calls
fn builtin_str_get(span: Span, args: &[Expr], type_table: &TypeTable, func_table: &FuncTable, var_table: &mut VarTable) -> Result<Typed<TBuiltinFnCall>, Error> {
    // make sure there's at least two arguments
    if args.len() < 2 {
        return Err(Error::BuiltinLittleArgs {
            call_span: span,
            min: 2..3,
        });
    }

    // make sure there's only two arguments
    if args.len() > 2 {
        return Err(Error::BuiltinManyArgs {
            call_span: span,
            max: 2..3,
            arg_span: args[2].span.clone(),
        });
    }

    // wrap the str expr
    let (str_expr, _) = wrap_expr(&args[0].asa, type_table, func_table, var_table)?;

    // make sure it's of type string
    let Type::String = str_expr.1
    else {
        return Err(Error::BuiltinArgTypeMismatch {
            span: str_expr.0.1,
            param_type: Type::String,
            arg_type: str_expr.1,
            call_span: span,
        });
    };

    // wrap the index expr and make sure it's of type nubmer
    let (idx_expr, _) = wrap_expr(&args[1].asa, type_table, func_table, var_table)?;
    match idx_expr.1 {
        Type::Number => (),
        _ => return Err(Error::BuiltinArgTypeMismatch {
            span: idx_expr.0.1,
            param_type: Type::Number,
            arg_type: idx_expr.1,
            call_span: span,
        }),
    }

    // return completed builtin-fn call
    Ok((
        TBuiltinFnCall::StringGet {
            span,
            string: str_expr.0,
            idx: idx_expr.0,
        },
        Type::String,
    ))
}
