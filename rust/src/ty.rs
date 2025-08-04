use {crate::*, std::fmt::Display};

#[derive(Clone, PartialEq, Eq)]
pub enum Ty {
    Bool,
    Int,
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ty::Bool => write!(f, "bool"),
            Ty::Int => write!(f, "int"),
        }
    }
}

#[derive(Clone)]
pub enum Data {
    Bool(bool),
    Int(i64),
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Bool(value) => write!(f, "{}", value),
            Data::Int(value) => write!(f, "{}", value),
        }
    }
}
