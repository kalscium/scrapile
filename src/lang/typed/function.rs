use ketchup::Span;
use crate::lang::{error::typed::Error, parser::expr::Expr, typed::expr, Spanned};
use super::{expr::TExpr, symbol_table::{FuncTable, TypeTable, VarTable}, types::{Type, Typed}};

/// A function type signature
#[derive(Debug, Clone)]
pub struct FuncSignature {
    pub params: Vec<Spanned<(String, Type)>>,
    pub retrn_type: Spanned<Type>,
}

/// Add type annotations to function calls
pub fn wrap_call(
    ident: &str,
    ident_span: Span,
    span: Span,
    args: &[Expr],
    type_table: &TypeTable,
    func_table: &FuncTable,
    var_table: &mut VarTable,
) -> Result<Typed<TExpr>, Error> {
    // verify that the function exists (and get the function signature)
    let Some(signature) = func_table.0.get(ident)
    else {
        return Err(Error::FuncNotFound {
            ident_span,
            ident: ident.to_string(),
            call_span: span,
        });
    };

    // check the length of the arguments
    if args.len() != signature.0.params.len() {
        return Err(Error::CallArgsAmount {
            call_span: span,
            amount: signature.0.params.len(),
            given_amount: args.len(),
            param_span: signature.1.clone(),
        })
    }

    let mut call_args = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        // wrap the argument in types
        let wrapped = expr::wrap_expr(&arg.asa, type_table, func_table, var_table)?.0;

        // get the parameter
        let param = &signature.0.params[i];

        // type-check the argument against the parameter
        if wrapped.1 != param.0.1 {
            return Err(Error::FuncCallTypeMismatch {
                param_span: param.1.clone(),
                func_span: signature.1.clone(),
                call_span: span.clone(),
                arg_span: arg.span.clone(),
                arg_type: wrapped.1,
                param_type: param.0.1.clone(),
            })
        }

        // convert the parameter passing into a variable set
        let ident = format!("$func${ident}/{}", param.0.0);
        call_args.push((ident, wrapped));
    }
    
    Ok((TExpr::Call(ident.to_string(), call_args), signature.0.retrn_type.0.clone()))
}
