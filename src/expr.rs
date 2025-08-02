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
        value: Value,
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
}

impl Expr {
    pub fn peephole_optimize(&self) -> Expr {
        match self {
            Expr::Add { lhs, rhs } => {
                let lhs = lhs.borrow().clone();
                let rhs = rhs.borrow().clone();
                match (lhs, rhs) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        if let (Data::Int(lhs), Data::Int(rhs)) = (lhs.data, rhs.data) {
                            Expr::Constant {
                                value: Value {
                                    ty: Ty::Int,
                                    data: Data::Int(lhs + rhs),
                                },
                            }
                        } else {
                            self.clone()
                        }
                    }
                    _ => self.clone(),
                }
            }
            Expr::Sub { lhs, rhs } => {
                let lhs = lhs.borrow().clone();
                let rhs = rhs.borrow().clone();
                match (lhs, rhs) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        if let (Data::Int(lhs), Data::Int(rhs)) = (lhs.data, rhs.data) {
                            Expr::Constant {
                                value: Value {
                                    ty: Ty::Int,
                                    data: Data::Int(lhs - rhs),
                                },
                            }
                        } else {
                            self.clone()
                        }
                    }
                    _ => self.clone(),
                }
            }
            Expr::Mul { lhs, rhs } => {
                let lhs = lhs.borrow().clone();
                let rhs = rhs.borrow().clone();
                match (lhs, rhs) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        if let (Data::Int(lhs), Data::Int(rhs)) = (lhs.data, rhs.data) {
                            Expr::Constant {
                                value: Value {
                                    ty: Ty::Int,
                                    data: Data::Int(lhs * rhs),
                                },
                            }
                        } else {
                            self.clone()
                        }
                    }
                    _ => self.clone(),
                }
            }
            Expr::Div { lhs, rhs } => {
                let lhs = lhs.borrow().clone();
                let rhs = rhs.borrow().clone();
                match (lhs, rhs) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        if let (Data::Int(lhs), Data::Int(rhs)) = (lhs.data, rhs.data) {
                            Expr::Constant {
                                value: Value {
                                    ty: Ty::Int,
                                    data: Data::Int(lhs / rhs),
                                },
                            }
                        } else {
                            self.clone()
                        }
                    }
                    _ => self.clone(),
                }
            }
            Expr::Neg { expr } => {
                let expr = expr.borrow().clone();
                match expr {
                    Expr::Constant { value, .. } => {
                        if let Data::Int(value) = value.data {
                            Expr::Constant {
                                value: Value {
                                    ty: Ty::Int,
                                    data: Data::Int(-value),
                                },
                            }
                        } else {
                            self.clone()
                        }
                    }
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
        }
    }
}
