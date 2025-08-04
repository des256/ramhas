use {crate::*, std::fmt::Display};

pub struct Constant {
    value: i64,
}

impl Constant {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Expr for Constant {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
