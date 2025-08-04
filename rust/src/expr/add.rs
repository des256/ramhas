use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Add {
    lhs: Rc<RefCell<dyn Expr>>,
    rhs: Rc<RefCell<dyn Expr>>,
}

impl Add {
    pub fn new(lhs: Rc<RefCell<dyn Expr>>, rhs: Rc<RefCell<dyn Expr>>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expr for Add {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.lhs.borrow(), self.rhs.borrow())
    }
}

/*
let lhs = lhs.borrow().peephole_optimize();
let rhs = rhs.borrow().peephole_optimize();
match (lhs, rhs) {
    // constant + constant -> constant
    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
        Expr::Constant { value: lhs + rhs }
    }
    // 0 + expr -> expr
    (Expr::Constant { value: lhs, .. }, rhs) => {
        if lhs == 0 {
            rhs.peephole_optimize()
        } else {
            self.clone()
        }
    }
    // expr + 0 -> expr
    (lhs, Expr::Constant { value: rhs, .. }) => {
        if rhs == 0 {
            lhs.peephole_optimize()
        } else {
            self.clone()
        }
    }
    // TODO: const + expr -> expr + const
    // TODO: const1 + (expr + const2) -> expr + (const1 + const2)
    // TODO: (expr1 + const) + expr2 -> (expr1 + expr2) + const
    // TODO: expr + expr -> expr * 2
    _ => self.clone(),
}
*/
