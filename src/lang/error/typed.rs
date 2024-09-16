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

    /// Occurs when you try to run a builtin function that doesn't exist
    BuiltinNotFound {
        /// The span of the builtin function ident
        ident_span: Span,

        /// The identifier of the builtin function
        ident: String,

        /// The span of the builtin function call
        call_span: Span,
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
}

impl Reportable for Error {
    fn report(&self, src_id: &str, src: &str) {
        use Error as E;

        let report = Report::build(ReportKind::Error, src_id, 10);

        let (msg, span, label, ctx_span, ctx_label) = match self {
            E::ArithmeticNonNumber { oper_span, oper_type, value_span, value_type } => ("cannot perform mathmatical operations on non-number types", oper_span, format!("cannot perform an {oper_type} operation an expr of type `{value_type}`"), value_span, format!("expected an expr of type `num`, instead found an expr of type `{value_type}`")),
            E::ConcatNonString { oper_span, value_span, value_type } => ("cannot concatinate non-string types", oper_span, format!("cannot concat an expr of type `{value_type}`"), value_span, format!("expected an expr of type `str`, instead found an expr of type `{value_type}`")),
            E::BuiltinManyArgs { call_span, max, arg_span } => ("unexpected builtin-function argument (too many args)", arg_span, "unexpected argument".to_string(), call_span, format!("builtin-func only expected {max:?} args")),
            E::BuiltinLittleArgs { call_span, min } => ("expected a builtin-function argument (too little args)", &(call_span.end-1..call_span.end), "expected an argument".to_string(), call_span, format!("builtin-func expected {min:?} args")),
            E::BuiltinWrongType { call_span, expected, arg_type, arg_span } => ("builtin function's argument is of an incorrect type", arg_span, format!("expected an expr of type `{expected}`, instead found an expr of type `{arg_type}`",), call_span, "occured in this builtin-func call".to_string()),
            E::MultipleMain { first_span, additional_span } => ("multiple main procedure definitions are not allowed", additional_span, "unexpected second main procedure definition".to_string(), first_span, "first main procedure defined here".to_string()),
            E::TypeNotFound { span } => ("type not found", span, "this type was not found in the project".to_string(), span, "it may be a typo or otherwise consider adding it or importing it".to_string()),
            E::VarTypeMismatch { span, type_span, expr_type, var_type } => ("variable assigned to with a value of the wrong type", span, format!("this expr is of the wrong type, expected an expr of type `{var_type}`, instead found an expr of type `{expr_type}`"), type_span, format!("variable's type `{var_type}` determined here")),
            E::VarNotFound { span } => ("variable not found", span, "this variable was not found in the current scope".to_string(), span, "it may be a typo or otherwise consider adding a variable of that name".to_string()),
            E::AssignToImmutable { var_span, span } => ("assignment to an immutable variable", span, "invalid mutation to a variable that is immutable".to_string(), var_span, "variable declared here, consider adding the `mut` keyword after `let` to make it mutable".to_string()),
            E::ListElementTypeMismatch { first_span, first_type, el_span, el_type } => ("list element's type doesn't match the type of the list", el_span, format!("expected an element of type `{first_type}`, instead found an element of type `{el_type}`"), first_span, format!("list is of type `{first_type}` due to the first element's type")),

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
                    .with_help("available builtin-funcs include: 'println'")
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
