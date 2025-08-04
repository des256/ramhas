use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Divide {
    lhs: Rc<RefCell<dyn Expr>>,
    rhs: Rc<RefCell<dyn Expr>>,
}

impl Divide {
    pub fn new(lhs: Rc<RefCell<dyn Expr>>, rhs: Rc<RefCell<dyn Expr>>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expr for Divide {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Divide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.lhs.borrow(), self.rhs.borrow())
    }
}

/*
let lhs = lhs.borrow().peephole_optimize();
let rhs = rhs.borrow().peephole_optimize();
match (lhs, rhs) {
    // constant / constant = constant
    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
        Expr::Constant { value: lhs / rhs }
    }
    // expr / 1 -> expr
    (lhs, Expr::Constant { value: rhs, .. }) => {
        if rhs == 1 {
            lhs.peephole_optimize()
        } else {
            self.clone()
        }
    }
    _ => self.clone(),
}
 */
