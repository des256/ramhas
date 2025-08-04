use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Multiply {
    lhs: Rc<RefCell<dyn Expr>>,
    rhs: Rc<RefCell<dyn Expr>>,
}

impl Multiply {
    pub fn new(lhs: Rc<RefCell<dyn Expr>>, rhs: Rc<RefCell<dyn Expr>>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expr for Multiply {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Multiply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.lhs.borrow(), self.rhs.borrow())
    }
}

/*
let lhs = lhs.borrow().peephole_optimize();
let rhs = rhs.borrow().peephole_optimize();
match (lhs, rhs) {
    // constant * constant -> constant
    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
        Expr::Constant { value: lhs * rhs }
    }
    // 1 * expr -> expr
    (Expr::Constant { value: lhs, .. }, rhs) => {
        if lhs == 1 {
            rhs.peephole_optimize()
        } else {
            self.clone()
        }
    }
    // expr * 1 -> expr
    (lhs, Expr::Constant { value: rhs, .. }) => {
        if rhs == 1 {
            lhs.peephole_optimize()
        } else {
            self.clone()
        }
    }
    // TODO: const * expr -> expr * const
    _ => self.clone(),
}
 */
