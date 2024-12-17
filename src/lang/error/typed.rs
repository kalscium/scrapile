use ariadne::{Color, Label, Report, ReportKind, Source};
use ketchup::Span;
use crate::lang::typed::types::Type;
use super::Reportable;

/// Type errors for scrapile
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Occurs when you try to perform maths on a non-number
    ArithmeticNonNumber {
        /// The span of the oper symbol
        oper_span: Span,

        /// The type of operation
        oper_type: &'static str,

        /// The span of the non-number
        value_span: Span,

        /// The type of the non-number
        value_type: Type,
    },

    /// Occurs when you try to concat a non-string
    ConcatNonString {
        /// The span of the concat operator
        oper_span: Span,

        /// The span of the non-string
        value_span: Span,

        /// The type of the non-string
        value_type: Type,
    },

    /// Occurs when you try to perform boolean operations on non-booleans
    NotBoolean {
        /// The span of the boolean operator
        oper_span: Span,

        /// The span of the non-bool
        value_span: Span,

        /// The type of the non-bool
        value_type: Type,
    },

    /// Occurs when you try to perform an operation on two values of different types
    OperationTypeMismatch {
        /// The span of the lhs
        lhs_span: Span,

        /// The type of the lhs
        lhs_type: Type,

        /// The span of the operator
        oper_span: Span,

        /// The span of the rhs
        rhs_span: Span,

        /// The type of the rhs
        rhs_type: Type,
    },

    /// Occurs when you try to run a builtin function that doesn't exist
    BuiltinNotFound {
        /// The span of the builtin function ident
        ident_span: Span,

        /// The identifier of the builtin function
        ident: String,

        /// The span of the builtin function call
        call_span: Span,
    },

    /// Occurs when you try to call a function that doesn't exist
    FuncNotFound {
        /// The span of the function ident
        ident_span: Span,

        /// The identifier of the function
        ident: String,

        /// The span of the function call
        call_span: Span,
    },

    /// Occurs when a function call's argument is of the wrong type
    FuncCallTypeMismatch {
        /// The span of the function's parameter
        param_span: Span,
        /// The span of the function definition
        func_span: Span,
        /// The span of the function call
        call_span: Span,
        /// The span of the call's argument
        arg_span: Span,
        /// The type of the argument expr
        arg_type: Type,
        /// The type of the parameter
        param_type: Type,
    },

    /// Occurs when you try to run a builtin function with too many arguments
    BuiltinManyArgs {
        /// The span of the builtin-function call
        call_span: Span,

        /// The max arguments a builtin-fn-call can have
        max: std::ops::Range<usize>,

        /// The argument's span
        arg_span: Span,
    },

    /// Occurs when you try to run a builtin function with too little arguments
    BuiltinLittleArgs {
        /// The span of the builtin-function call
        call_span: Span,

        /// The min arguments a builtin-fn-call can have
        min: std::ops::Range<usize>,
    },

    /// Occurs when you try to call a function with the incorrect amount of arguments
    CallArgsAmount {
        /// The span of the function call
        call_span: Span,
        /// The expected amount of arguments
        amount: usize,
        /// The amount of arguments given
        given_amount: usize,
        /// The span of the parameter definition
        param_span: Span,
    },

    /// Occurs when there is an incorrectly typed argument o a builtin function
    BuiltinWrongType {
        /// The span of the builtin-function 
        call_span: Span,

        /// The expected type
        expected: Type,

        /// The type of the argument
        arg_type: Type,

        /// The span of the argument
        arg_span: Span,
    },

    /// Occurs when there are multiple main procedures that are defined in the same project
    MultipleMain {
        /// The span of the first main block
        first_span: Span,
        
        /// The span of the additional main block
        additional_span: Span,
    },

    /// Occurs when there are multiple function definitions with the same identifier
    MultipleFunc {
        /// The span of the first definition
        first_span: Span,
        /// The span of the additional definition
        additional_span: Span,
    },

    /// Occurs when there is no main procedure defined in the project
    NoMain,

    /// Occurs when a type is used that does not exist
    TypeNotFound {
        /// The span of the type being used
        span: Span,
    },

    /// Occurs when you try to assign a value of the wrong type to a variable
    VarTypeMismatch {
        /// The span of the expr
        span: Span,
        /// The span of the type declaration of the variable
        type_span: Span,
        /// The type of the expr
        expr_type: Type,
        /// The type of the variable
        var_type: Type,
    },

    /// Occurs when you try to assign a value of the wrong type to a variable
    RetrnTypeMismatch {
        /// The span of the expr
        span: Span,
        /// The span of the type declaration of the variable
        type_span: Span,
        /// The type of the expr
        expr_type: Type,
        /// The type of the variable
        retrn_type: Type,
    },

    /// Occurs when you try to get the value of a variable that doesn't exist
    VarNotFound {
        /// The span of the variable use
        span: Span,
    },

    /// Occurs when you try to assign to an immutable variable
    AssignToImmutable {
        /// The span of the variable itself
        var_span: Span,
        /// The span of the mutation statement
        span: Span,
    },

    /// Occurs when there is a type mismatch within a list (two elements of different types)
    ListElementTypeMismatch {
        /// The span of the first element
        first_span: Span,
        /// The first element's type
        first_type: Type,
        /// The span of the invalid element
        el_span: Span,
        /// The type of the invalid element
        el_type: Type,
    },
    
    /// Occurs when you try to use a non-boolean expr as the condition for an 'if' or 'while' statement
    NonBoolCond {
        /// The span of the non-bool
        span: Span,
        /// The type of the non-bool
        expr_type: Type,
        /// The span of the 'if' or 'while' statement
        ctx_span: Span,
    },

    /// Occurs when you pass an argument of the wrong type to a builtin function
    BuiltinArgTypeMismatch {
        /// The span of the argument
        span: Span,
        /// The expected type of the arugment
        param_type: Type,
        /// The type of the argument
        arg_type: Type,
        /// The span of the builtin-call
        call_span: Span,
    },
}

impl Reportable for Error {
    fn report(&self, src_id: &str, src: &str) {
        use Error as E;

        let report = Report::build(ReportKind::Error, src_id, 10);

        let (msg, span, label, ctx_span, ctx_label) = match self {
            E::ArithmeticNonNumber { oper_span, oper_type, value_span, value_type } => ("cannot perform mathmatical operations on non-number types", oper_span, format!("cannot perform an {oper_type} operation an expr of type `{value_type}`"), value_span, format!("expected an expr of type `num`, instead found an expr of type `{value_type}`")),
            E::ConcatNonString { oper_span, value_span, value_type } => ("cannot concatinate non-string types", oper_span, format!("cannot concat an expr of type `{value_type}`"), value_span, format!("expected an expr of type `str`, instead found an expr of type `{value_type}`")),
            E::NotBoolean { oper_span, value_span, value_type } => ("cannot perform boolean operations on non-booleans", oper_span, format!("cannot perform bool-oper on an expr of type `{value_type}`"), value_span, format!("expected an expr of type `bool`, instead found an expr of type `{value_type}`")),
            E::BuiltinManyArgs { call_span, max, arg_span } => ("unexpected builtin-function argument (too many args)", arg_span, "unexpected argument".to_string(), call_span, format!("builtin-func only expected {max:?} args")),
            E::BuiltinLittleArgs { call_span, min } => ("expected a builtin-function argument (too little args)", &(call_span.end-1..call_span.end), "expected an argument".to_string(), call_span, format!("builtin-func expected {min:?} args")),
            E::CallArgsAmount { call_span, amount, given_amount, param_span } => ("function called with an incorrect amount of arguments", call_span, format!("expected {amount} arguments, found {given_amount} instead"), param_span, "function parameters defined here".to_string()),
            E::BuiltinWrongType { call_span, expected, arg_type, arg_span } => ("builtin function's argument is of an incorrect type", arg_span, format!("expected an expr of type `{expected}`, instead found an expr of type `{arg_type}`",), call_span, "occured in this builtin-func call".to_string()),
            E::MultipleMain { first_span, additional_span } => ("multiple main procedure definitions are not allowed", additional_span, "unexpected second main procedure definition".to_string(), first_span, "first main procedure defined here".to_string()),
            E::MultipleFunc { first_span, additional_span } => ("function was defined multiple times", additional_span, "unexpected second definition".to_string(), first_span, "function was defined first here".to_string()),
            E::TypeNotFound { span } => ("type not found", span, "this type was not found in the project".to_string(), span, "it may be a typo or otherwise consider adding it or importing it".to_string()),
            E::FuncNotFound { ident_span, ident, call_span } => ("called function not found in scope", ident_span, format!("no function called '{ident}' found"), call_span, "in this function call".to_string()),
            E::VarTypeMismatch { span, type_span, expr_type, var_type } => ("variable assigned to with a value of the wrong type", span, format!("this expr is of the wrong type, expected an expr of type `{var_type}`, instead found an expr of type `{expr_type}`"), type_span, format!("variable's type `{var_type}` determined here")),
            E::RetrnTypeMismatch { span, type_span, expr_type, retrn_type } => ("function body-block returns an expr of the wrong type", span, format!("this expr is of the wrong type, expected an expr of type `{retrn_type}`, instead found an expr of type `{expr_type}`"), type_span, format!("function's return-type `{retrn_type}` defined here")),
            E::VarNotFound { span } => ("variable not found", span, "this variable was not found in the current scope".to_string(), span, "it may be a typo or otherwise consider adding a variable of that name".to_string()),
            E::AssignToImmutable { var_span, span } => ("assignment to an immutable variable", span, "invalid mutation to a variable that is immutable".to_string(), var_span, "variable declared here, consider adding the `mut` keyword after `let` to make it mutable".to_string()),
            E::ListElementTypeMismatch { first_span, first_type, el_span, el_type } => ("list element's type doesn't match the type of the list", el_span, format!("expected an element of type `{first_type}`, instead found an element of type `{el_type}`"), first_span, format!("list is of type `{first_type}` due to the first element's type")),
            E::NonBoolCond { span, expr_type, ctx_span } => ("invalid non-boolean condition for 'if'/'while' statement", span, format!("expr is of type `{expr_type}`, expected an expr of type `bool`"), ctx_span, "part of this 'if' statement".to_string()),
            E::BuiltinArgTypeMismatch { span, arg_type, param_type, call_span } => ("argument to builtin-function call is of the wrong type", span, format!("expected an expr of type `{param_type}`, instead found an expr of type `{arg_type}`"), call_span, "in this builtin-func call".to_string()),
            
            E::NoMain => {
                return report
                    .with_message("no main procedure found")
                    .with_label(
                        Label::new((src_id, 0..0))
                            .with_message("expected a main procedure definition")
                        .with_color(Color::Red),
                    )
                    .with_help("you could try defining a main procedure like so `main { ... }`")
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap();
            },

            E::BuiltinNotFound { ident_span, ident, call_span } => {
                return report
                    .with_message("invalid builtin-function call")
                    .with_label(
                        Label::new((src_id, call_span.clone()))
                            .with_message("this builtin-func call is invalid")
                            .with_color(Color::Red),
                    )
                    .with_label(
                        Label::new((src_id, ident_span.clone()))
                            .with_message(format!("no builtin-func was found with the name '{ident}'"))
                            .with_color(Color::BrightBlue),
                    )
                    .with_help("available builtin-funcs include: 'println', 'as_str', 'input', 'timer', 'panic', 'list_len', 'list_get', 'list_push' and 'list_insert'")
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap();
            },

            E::FuncCallTypeMismatch { func_span, param_span, call_span, arg_span, arg_type, param_type } => {
                return report
                    .with_message("function called with an argument of the wrong type")
                    .with_label(
                        Label::new((src_id, arg_span.clone()))
                            .with_message("argument is of the wrong type")
                            .with_message(format!("expected an argument of type '{param_type}', found an expr of type '{arg_type}'"))
                            .with_color(Color::Red),
                    )
                    .with_label(
                        Label::new((src_id, call_span.clone()))
                            .with_message("in this function call")
                            .with_color(Color::BrightBlue),
                    )
                    .with_label(
                        Label::new((src_id, param_span.clone()))
                            .with_message("parameter type defined here")
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new((src_id, func_span.clone()))
                            .with_message("due to this function definition")
                            .with_color(Color::BrightBlue),
                    )
                .finish()
                .eprint((src_id, Source::from(src)))
                .unwrap();
            },

            E::OperationTypeMismatch { lhs_span, lhs_type, oper_span, rhs_span, rhs_type } => {
                return report
                    .with_message("type mismatch between the left and right sides of an operation")
                    .with_label(
                        Label::new((src_id, lhs_span.clone()))
                            .with_message(format!("expected a `{lhs_type}` due to the type of this expr"))
                            .with_color(Color::BrightBlue),
                    )
                    .with_label(
                        Label::new((src_id, oper_span.clone()))
                            .with_message(format!("cannot operate upon two values of different types"))
                            .with_color(Color::Red),
                    )
                    .with_label(
                        Label::new((src_id, rhs_span.clone()))
                            .with_message(format!("expected an expr of type `{lhs_type}`, instead found a value of type `{rhs_type}`"))
                            .with_color(Color::BrightBlue),
                    )
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap();
            },
        };

        report
            .with_message(msg)
            .with_label(
                Label::new((src_id, span.clone()))
                    .with_message(label)
                    .with_color(Color::Red),
            )
            .with_label(
                Label::new((src_id, ctx_span.clone()))
                    .with_message(ctx_label)
                    .with_color(Color::BrightBlue),
            )
            .finish()
            .eprint((src_id, Source::from(src)))
            .unwrap();
    }
}
