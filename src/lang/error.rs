#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Error {
    #[default]
    UnexpectedCharacter,
    UnclosedParentheses,
    UnexpectedToken,
    ExpectedIdent,
}
