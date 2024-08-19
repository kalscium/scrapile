use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use ketchup::{error::KError, Span};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Error {
    #[default]
    UnexpectedCharacter,

    UnexpectedToken,
    ExpectedIdent,
    ExpectedStmt,
    ExpectedExpr,

    UnclosedParentheses {
        paren_start_span: Span,
    },
    UnclosedBrace {
        brace_start_span: Span,
    },
    ExpectedCommaOrRParen {
        tuple_start_span: Span,
    },
    ExpectedSemiOrRBrace {
        block_start_span: Span,
    },

    ExpectedCallLParen {
        ident_start_span: Span,
    },

    ExpectedBlockForMain {
        main_start_span: Span,
    },
}

pub trait Reportable {
    fn report(&self, src_id: &str, src: &str);
}

impl Reportable for KError<Error> {
    fn report(&self, src_id: &str, src: &str) {
        use KError as K;
        use Error as E;

        let report = Report::build(ReportKind::Error, src_id, 10);

        let (msg, label) = match self {
            K::DoubleSpaceConflict { .. } => ("expected an expression", "found this instead"),
            K::UnexpectedOper(_) => ("unexpected operation", "unexpected operation"),
            K::ExpectedOper { .. } => ("expected operation", "found this instead"),

            K::Other(span, other) => match other {
                E::UnexpectedCharacter => ("unexpected or invalid character", "unexpected character"),
                E::UnexpectedToken => ("unexpected token", "unexpected token"),
                E::ExpectedIdent => ("expected identifier", "found this instead"),
                E::ExpectedStmt => ("expected statement", "found this instead"),
                E::ExpectedExpr => ("expected an expression", "found this instead"),

                // errors with multiple labels

                E::UnclosedParentheses { paren_start_span } => return report
                    .with_message("unclosed parentheses")
                    .with_label(
                        Label::new((src_id, span.clone()))
                            .with_message("expected `)`")
                            .with_color(Color::Red)
                    )
                    .with_label(
                        Label::new((src_id, paren_start_span.clone()))
                            .with_message("to complete this tuple")
                            .with_color(Color::Blue)
                    )
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap(),
                E::UnclosedBrace { brace_start_span } => return report
                    .with_message("unclosed brace")
                    .with_label(
                        Label::new((src_id, span.clone()))
                            .with_message("expected `}`")
                            .with_color(Color::Red)
                    )
                    .with_label(
                        Label::new((src_id, brace_start_span.clone()))
                            .with_message("to complete this block")
                            .with_color(Color::Blue)
                    )
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap(),

                E::ExpectedCommaOrRParen { tuple_start_span } => return report
                    .with_message("expected comma or right parenthesis")
                    .with_label(
                        Label::new((src_id, span.clone()))
                            .with_message("expected `,` or `)`")
                            .with_color(Color::Red)
                    )
                    .with_label(
                        Label::new((src_id, tuple_start_span.clone()))
                            .with_message("to continue or complete this tuple")
                            .with_color(Color::Blue)
                    )
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap(),
                E::ExpectedSemiOrRBrace { block_start_span } => return report
                    .with_message("expected semi-colon or right brace")
                    .with_label(
                        Label::new((src_id, span.clone()))
                            .with_message("expected `;` or `}`")
                            .with_color(Color::Red)
                    )
                    .with_label(
                        Label::new((src_id, block_start_span.clone()))
                            .with_message("to continue or complete this block")
                            .with_color(Color::Blue)
                    )
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap(),

                E::ExpectedCallLParen { ident_start_span } => return report
                    .with_message("expected arguments for this function call")
                    .with_label(
                        Label::new((src_id, span.clone()))
                            .with_message("expected arguments `(`")
                            .with_color(Color::Red)
                    )
                    .with_label(
                        Label::new((src_id, ident_start_span.clone()))
                            .with_message("this func call expected args")
                            .with_color(Color::Blue)
                    )
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap(),
                E::ExpectedBlockForMain { main_start_span } => return report
                    .with_message("expected a body for the main procedure")
                    .with_label(
                        Label::new((src_id, span.clone()))
                            .with_message("expected body `{`")
                            .with_color(Color::Red)
                    )
                    .with_label(
                        Label::new((src_id, main_start_span.clone()))
                            .with_message(format!("expected due to `{}` keyword", "main".fg(Color::Cyan)))
                            .with_color(Color::Blue)
                    )
                    .finish()
                    .eprint((src_id, Source::from(src)))
                    .unwrap(),
            },
        };

        report
            .with_message(msg)
            .with_label(
                Label::new((src_id, self.span().clone()))
                    .with_message(label)
                    .with_color(Color::Red)
            )
            .finish()
            .eprint((src_id, Source::from(src)))
            .unwrap();
    }
}
