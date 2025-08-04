use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Subtract {
    lhs: Rc<RefCell<dyn Expr>>,
    rhs: Rc<RefCell<dyn Expr>>,
}

impl Subtract {
    pub fn new(lhs: Rc<RefCell<dyn Expr>>, rhs: Rc<RefCell<dyn Expr>>) -> Self {
        Self { lhs, rhs }
    }
}

impl Expr for Subtract {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Subtract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} - {})", self.lhs.borrow(), self.rhs.borrow())
    }
}

/*
let lhs = lhs.borrow().peephole_optimize();
let rhs = rhs.borrow().peephole_optimize();
match (lhs, rhs) {
    // constant - constant -> constant
    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
        Expr::Constant { value: lhs - rhs }
    }
    // 0 - expr -> -expr
    (Expr::Constant { value: lhs, .. }, rhs) => {
        if lhs == 0 {
            Expr::Neg {
                expr: Rc::new(RefCell::new(rhs.peephole_optimize())),
            }
        } else {
            self.clone()
        }
    }
    // expr - 0 -> expr
    (lhs, Expr::Constant { value: rhs, .. }) => {
        if rhs == 0 {
            lhs.peephole_optimize()
        } else {
            self.clone()
        }
    }
    // TODO: expr - expr -> 0
    _ => self.clone(),
}
 */
