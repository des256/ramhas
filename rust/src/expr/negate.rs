use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Negate {
    expr: Rc<RefCell<dyn Expr>>,
}

impl Negate {
    pub fn new(expr: Rc<RefCell<dyn Expr>>) -> Self {
        Self { expr }
    }
}

impl Expr for Negate {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Negate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-({})", self.expr.borrow())
    }
}

/*
let expr = expr.borrow().peephole_optimize();
match expr {
    // -constant -> constant
    Expr::Constant { value, .. } => Expr::Constant { value: -value },
    _ => self.clone(),
}
 */
