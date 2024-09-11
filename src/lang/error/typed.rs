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
}

impl Reportable for Error {
    fn report(&self, src_id: &str, src: &str) {
        use Error as E;

        let report = Report::build(ReportKind::Error, src_id, 10);

        let (msg, span, label, ctx_span, ctx_label) = match self {
            E::ArithmeticNonNumber { oper_span, oper_type, value_span, value_type } => ("cannot perform mathmatical operations on non-number types".to_string(), oper_span, format!("cannot perform an {oper_type} operation an expr of type `{value_type}`"), value_span, format!("expected an expr of type `num`, instead found an expr of type `{value_type}`")),
            E::ConcatNonString { oper_span, value_span, value_type } => ("cannot concatinate non-string types".to_string(), oper_span, format!("cannot concat an expr of type `{value_type}`"), value_span, format!("expected an expr of type `str`, instead found an expr of type `{value_type}`")),
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
