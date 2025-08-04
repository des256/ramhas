use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct NotEqual {
    lhs: Rc<RefCell<dyn Expr>>,
    rhs: Rc<RefCell<dyn Expr>>,
}

impl NotEqual {
    pub fn new(lhs: Rc<RefCell<dyn Expr>>, rhs: Rc<RefCell<dyn Expr>>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expr for NotEqual {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for NotEqual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} != {})", self.lhs.borrow(), self.rhs.borrow())
    }
}
