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
    }
}

impl Reportable for Error {
    fn report(&self, src_id: &str, src: &str) {
        use Error as E;

        let report = Report::build(ReportKind::Error, src_id, 10);

        let (msg, span, label, ctx_span, ctx_label) = match self {
            E::ArithmeticNonNumber { oper_span, oper_type, value_span, value_type } => ("cannot perform mathmatical operations on non-number types", oper_span, format!("cannot perform an {oper_type} operation an expr of type `{value_type}`"), value_span, format!("expected an expr of type `num`, instead found an expr of type `{value_type}`")),
            E::ConcatNonString { oper_span, value_span, value_type } => ("cannot concatinate non-string types", oper_span, format!("cannot concat an expr of type `{value_type}`"), value_span, format!("expected an expr of type `str`, instead found an expr of type `{value_type}`")),
            E::BuiltinManyArgs { call_span, max, arg_span } => ("unexpected builtin-function argument (too many args)", arg_span, "unexpected argument".to_string(), call_span, format!("builtin-func only expected {max:?} args")),
            E::BuiltinWrongType { call_span, expected, arg_type, arg_span } => ("builtin function's argument is of an incorrect type", arg_span, format!("expected an expr of type `{expected}`, instead found an expr of type `{arg_type}`",), call_span, "occured in this builtin-func call".to_string()),

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
