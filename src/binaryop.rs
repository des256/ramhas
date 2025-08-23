use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    LogicalAnd,
    LogicalOr,
    ShiftLeft,
    ShiftRight,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::And => write!(f, "&"),
            BinaryOp::Or => write!(f, "|"),
            BinaryOp::Xor => write!(f, "^"),
            BinaryOp::LogicalAnd => write!(f, "&&"),
            BinaryOp::LogicalOr => write!(f, "||"),
            BinaryOp::ShiftLeft => write!(f, "<<"),
            BinaryOp::ShiftRight => write!(f, ">>"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::LessThan => write!(f, "<"),
            BinaryOp::GreaterThan => write!(f, ">"),
            BinaryOp::LessThanOrEqual => write!(f, "<="),
            BinaryOp::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}
