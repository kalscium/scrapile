use super::Expr;

/// A condition in scratch (different from an expression as it can only be used in if statements)
#[derive(Debug, Clone)]
pub enum Condition {
    // expr to expr conditinos
    MoreThan(Expr, Expr),
    LessThan(Expr, Expr),
    EqualTo(Expr, Expr),

    // condition to condition conditions
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}
