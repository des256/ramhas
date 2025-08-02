use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

#[derive(Clone)]
pub enum Ctrl {
    Start {
        args: Vec<Rc<RefCell<Expr>>>,
        scopes: Scopes,
    },
    Stop {
        ctrls: Vec<Rc<RefCell<Ctrl>>>,
    },
    Return {
        ctrl: Rc<RefCell<Ctrl>>,
        expr: Rc<RefCell<Expr>>,
    },
}

impl Ctrl {
    pub fn declare_symbol(&mut self, name: &str, expr: Rc<RefCell<Expr>>) {
        match self {
            Ctrl::Start { scopes, .. } => {
                scopes.declare(name, expr);
            }
            _ => panic!("declare_symbol: not supported"),
        }
    }

    pub fn set_symbol(&mut self, name: &str, expr: Rc<RefCell<Expr>>) {
        match self {
            Ctrl::Start { scopes, .. } => {
                scopes.set(name, expr);
            }
            _ => panic!("set_symbol: not supported"),
        }
    }

    pub fn symbol(&self, name: &str) -> Option<Rc<RefCell<Expr>>> {
        match self {
            Ctrl::Start { scopes, .. } => scopes.get(name),
            _ => panic!("symbol: not supported"),
        }
    }

    pub fn push_scope(&mut self) {
        match self {
            Ctrl::Start { scopes, .. } => {
                scopes.push();
            }
            _ => panic!("push_scope: not supported"),
        }
    }

    pub fn pop_scope(&mut self) {
        match self {
            Ctrl::Start { scopes, .. } => {
                scopes.pop();
            }
            _ => panic!("pop_scope: not supported"),
        }
    }
}

impl Display for Ctrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ctrl::Start { args, .. } => {
                write!(
                    f,
                    "Start({})",
                    args.iter()
                        .map(|a| a.borrow().to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
            Ctrl::Stop { ctrls } => {
                write!(
                    f,
                    "Stop({})",
                    ctrls
                        .iter()
                        .map(|c| format!("{:016x}", c.as_ptr() as u64))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
            Ctrl::Return { ctrl, expr } => {
                write!(f, "Return({:016x},{})", ctrl.as_ptr() as u64, expr.borrow())
            }
        }
    }
}
