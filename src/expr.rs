use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

#[derive(Clone)]
pub enum Expr {
    Proj {
        ctrl: Rc<RefCell<Ctrl>>,
        index: usize,
    },
    Constant {
        value: i64,
    },
    Add {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    Sub {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    Mul {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    Div {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    Neg {
        expr: Rc<RefCell<Expr>>,
    },
    Not {
        expr: Rc<RefCell<Expr>>,
    },
    Equal {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    NotEqual {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    LessThan {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    LessThanOrEqual {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    GreaterThan {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
    GreaterThanOrEqual {
        lhs: Rc<RefCell<Expr>>,
        rhs: Rc<RefCell<Expr>>,
    },
}

impl Expr {
    pub fn peephole_optimize(&self) -> Expr {
        match self {
            Expr::Add { lhs, rhs } => {
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
                    _ => self.clone(),
                }
            }
            Expr::Sub { lhs, rhs } => {
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
                    _ => self.clone(),
                }
            }
            Expr::Mul { lhs, rhs } => {
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
                    _ => self.clone(),
                }
            }
            Expr::Div { lhs, rhs } => {
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
            }
            Expr::Neg { expr } => {
                let expr = expr.borrow().peephole_optimize();
                match expr {
                    // -constant -> constant
                    Expr::Constant { value, .. } => Expr::Constant { value: -value },
                    _ => self.clone(),
                }
            }
            _ => self.clone(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Proj { ctrl, index } => {
                write!(f, "Proj({:016x},{})", ctrl.as_ptr() as u64, index)
            }
            Expr::Constant { value } => write!(f, "{}", value),
            Expr::Add { lhs, rhs } => {
                write!(f, "({} + {})", lhs.borrow(), rhs.borrow())
            }
            Expr::Sub { lhs, rhs } => {
                write!(f, "({} - {})", lhs.borrow(), rhs.borrow())
            }
            Expr::Mul { lhs, rhs } => {
                write!(f, "({} * {})", lhs.borrow(), rhs.borrow())
            }
            Expr::Div { lhs, rhs } => {
                write!(f, "({} / {})", lhs.borrow(), rhs.borrow())
            }
            Expr::Neg { expr } => {
                write!(f, "-({})", expr.borrow())
            }
            Expr::Not { expr } => {
                write!(f, "!({})", expr.borrow())
            }
            Expr::Equal { lhs, rhs } => {
                write!(f, "({} == {})", lhs.borrow(), rhs.borrow())
            }
            Expr::NotEqual { lhs, rhs } => {
                write!(f, "({} != {})", lhs.borrow(), rhs.borrow())
            }
            Expr::LessThan { lhs, rhs } => {
                write!(f, "({} < {})", lhs.borrow(), rhs.borrow())
            }
            Expr::LessThanOrEqual { lhs, rhs } => {
                write!(f, "({} <= {})", lhs.borrow(), rhs.borrow())
            }
            Expr::GreaterThan { lhs, rhs } => {
                write!(f, "({} > {})", lhs.borrow(), rhs.borrow())
            }
            Expr::GreaterThanOrEqual { lhs, rhs } => {
                write!(f, "({} >= {})", lhs.borrow(), rhs.borrow())
            }
        }
    }
}
