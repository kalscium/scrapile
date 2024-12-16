use ariadne::{Color, Label, Report, ReportKind, Source};
use ketchup::{error::KError, Span};
use super::Reportable;

/// Parsing errors for scrapile
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Error {
    /// Occurs when there is a character that the lexer does not recognise
    #[default]
    UnexpectedCharacter,

    /// Occurs when a statement is expected but not found
    ExpectedStmt,
    /// Occurs when a expression is expected but not found
    ExpectedExpr,
    /// Occurs when the parser expects a root token (like `main` or `fn`) but find an unexpected one instead
    ExpectedRoot,
    /// Occurs when there is a type usage that was expected but not found
    ExpectedType,

    /// Occurs when there is an unclosed parentheses
    UnclosedParentheses {
        /// The location of the start of the parentheses
        ctx_span: Span,
    },
    /// Occurs when there is an unclosed brace
    UnclosedBrace {
        /// The location of the start of the brace
        ctx_span: Span,
    },
    /// Occurs when there is an unclosed bracket
    UnclosedBrackets {
        /// The location of the start of the bracket
        ctx_span: Span,
    },
    /// Occurs when there is a token besides a comma or right-parenthesis
    ExpectedCommaOrRParen {
        /// The location of the start of the parentheses
        ctx_span: Span,
    },
    /// Occurs when there is a token besides a semi-colon or right-brace
    ExpectedSemiOrRBrace {
        /// The location of the start of the block
        ctx_span: Span,
    },
    /// Occurs when there is a token besides a comma or right-bracket
    ExpectedCommaOrRBracket {
        /// The location of the start of the list
        ctx_span: Span,
    },

    /// Occurs when args for a function are expected but not found
    ExpectedCallLParen {
        /// The location of the start of the parentheses
        ctx_span: Span,
    },

    /// Occurs when block body of the main procedure is expected but not found
    ExpectedBlockForMain {
        /// The location of the start of the parentheses
        ctx_span: Span,
    },

    /// Occurs when the `let` var declaration cannot find a `mut` or identifier
    ExpectedMutOrIdent {
        /// The location of the `let` var declaration
        ctx_span: Span,
    },
    /// Occurs when the `let` var declaration cannot find a `:` or `=`
    ExpectedColonOrEQ {
        /// The location of the `let` var declaration
        ctx_span: Span,
    },
    /// Occurs when the  var assignment cannot find a `=`
    ExpectedEQ {
        /// The location of the `let` var declaration
        ctx_span: Span,
    },

    /// Occurs when something expects an identifier but cannot find one
    ExpectedIdent {
        /// The location of the statement
        ctx_span: Span,
    },

    /// Occurs when an if statement doesn't have required parentheses around its condition
    ExpectedCondLParen {
        /// The location of the if or while statement
        ctx_span: Span,
    },
    
    /// Occurs when a function definition doesn't have an identifier
    ExpectedFnIdent {
        /// The location of the function def
        ctx_span: Span,
    },
    /// Occurs when a function parameter doesn't have an identifier
    ExpectedFnParamIdent {
        /// The location of the function parameters
        ctx_span: Span,
    },
    /// Occurs when a function parameter doesn't have an a colon to separate the identifier and type
    ExpectedFnParamColon {
        /// The location of the parameter
        ctx_span: Span,
    },
    /// Occurs when a function definition is missing a body
    ExpectedFnBody {
        /// The locatino of the function def
        ctx_span: Span,
    },
}

impl Reportable for KError<Error> {
    fn report(&self, src_id: &str, src: &str) {
        use KError as K;
        use Error as E;

        let report = Report::build(ReportKind::Error, src_id, 10);

        let (msg, span, label, ctx_span, ctx_label) = match self.clone() {
            K::DoubleSpaceConflict { ctx_span, span } => ("expected an expression", span, "found this instead", ctx_span, "expected an expr as an input"),
            K::UnexpectedOper { ctx_span, span } => ("unexpected operation", span, "unexpected operation", ctx_span, "did not expect an operation after this"),
            K::ExpectedOper { ctx_span, span, precedence: _ } => ("expected operation", span, "found this instead", ctx_span, "expected an expr after this"),

            K::Other(span, other) => match other {
                E::UnexpectedCharacter => ("unexpected or invalid character", span.clone(), "unexpected character", span, "consider removing this"),
                E::ExpectedStmt => ("expected statement", span.clone(), "found this instead", span, "consider removing this or inserting a statement"), // assuming it's an error caused by `;;`
                E::ExpectedExpr => ("expected an expression", span.clone(), "found this instead", span, "consider removing this or inserting an expression"),
                E::ExpectedRoot => ("expected root token", span.clone(), "expected a root token like `main` or `fn ...`", span, "consider wrapping this in a `main { ... }` or function"),
                E::ExpectedType => ("expected a type annotation", span.clone(), "expected a type annotation", span, "consider adding a type annotation here, like `str` or `num`"),

                E::UnclosedParentheses { ctx_span } => ("unclosed parentheses", span, "expected `)`", ctx_span, "to complete this"),
                E::UnclosedBrace { ctx_span } => ("unclosed brace", span, "expected `}`", ctx_span, "to complete this block"),
                E::UnclosedBrackets { ctx_span } => ("unclosed brackets", span, "expected `]`", ctx_span, "to complete this list"),
                E::ExpectedCommaOrRParen { ctx_span } => ("expected comma or `)`", span, "expected `,` or `)`", ctx_span, "to continue or complete this"),
                E::ExpectedSemiOrRBrace { ctx_span } => ("expected semi-colon or right brace", span, "expected `;` or `}`", ctx_span, "to continue or complete this block"),
                E::ExpectedCommaOrRBracket { ctx_span } => ("expected comma or `]`", span, "expected `,` or `]`", ctx_span, "to continue or complete this list"),
                E::ExpectedCallLParen { ctx_span } => ("expected arguments for this function call", span, "expected arguments `(`", ctx_span, "this func call expected args" ),
                E::ExpectedBlockForMain { ctx_span } => ("expected a body for the main procedure", span, "expected body `{`", ctx_span, "expected due to the `main` keyword"),

                E::ExpectedMutOrIdent { ctx_span } => ("expected either a `mut` keyword or identifier in `let` statement", span, "found this instead", ctx_span, "in this let statement"),
                E::ExpectedColonOrEQ { ctx_span } => ("expected either a `:` (for type annotations) or a `=` (for value definition) in let statement", span, "found this instead", ctx_span, "in this let statement"),
                E::ExpectedEQ { ctx_span } => ("expected a `=` to define a value in variable assignment statement", span, "found this instead", ctx_span, "in this var assign statement"),

                E::ExpectedIdent { ctx_span } => ("expected an identifier", span, "found this instead", ctx_span, "in this statement"),

                E::ExpectedCondLParen { ctx_span } => ("expected `(` in 'if'/'while' statement condition", span, "try wrapping this in parentheses", ctx_span, "in this 'if'/'while' statement"),

                E::ExpectedFnIdent { ctx_span } => ("expected identifier for function definition", span, "found this instead", ctx_span, "in these function parameters"),
                E::ExpectedFnParamIdent { ctx_span } => ("expected identifier for function parameter", span, "found this instead", ctx_span, "in these function parameters"),
                E::ExpectedFnParamColon { ctx_span } => ("expected `:` to separate the identifier and type", span, "found this instead", ctx_span, "in this function parameter"),
                E::ExpectedFnBody { ctx_span } => ("expected one of `->`, `(`, `{`", span, "found this instead", ctx_span, "in function definition"),
            },
        };

        report
            .with_message(msg)
            .with_label(
                Label::new((src_id, span))
                    .with_message(label)
                    .with_color(Color::Red),
            )
            .with_label(
                Label::new((src_id, ctx_span))
                    .with_message(ctx_label)
                    .with_color(Color::BrightBlue),
            )
            .finish()
            .eprint((src_id, Source::from(src)))
            .unwrap();
    }
}
