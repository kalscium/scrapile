use ariadne::{Color, Label, Report, ReportKind, Source};
use ketchup::Span;
use crate::lang::typed::types::Type;

use super::Reportable;

/// Type errors for scrapile
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Occurs when you try to negate (`-`) something besides a number
    CanOnlyNegNumber {
        /// The span of the symbol
        oper_span: Span,
        /// The span of the value you're trying to negate
        value_span: Span,
        /// The type of the value you're trying to negate
        value_type: Type,
    },

    /// Occurs when you try to pos (`+`) something besides a number
    CanOnlyPosNumber {
        /// The span of the symbol
        oper_span: Span,
        /// The span of the value you're trying to pos
        value_span: Span,
        /// The type of the value you're trying to pos
        value_type: Type,
    },
}

impl Reportable for Error {
    fn report(&self, src_id: &str, src: &str) {
        use Error as E;

        let report = Report::build(ReportKind::Error, src_id, 10);

        let (msg, span, label, ctx_span, ctx_label) = match self {
            E::CanOnlyNegNumber { oper_span: negate_span, value_span, value_type } => (format!("cannot negate value of type '{}'", value_type), negate_span, "expected a number to negate".to_string(), value_span, format!("instead found value of type '{}'", value_type)),
            E::CanOnlyPosNumber { oper_span: negate_span, value_span, value_type } => (format!("cannot pos value of type '{}'", value_type), negate_span, "expected a number to pos".to_string(), value_span, format!("instead found value of type '{}'", value_type)),
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
