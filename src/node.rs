use std::{cell::RefCell, fmt::Display, rc::Rc};

pub enum Resultlet {
    Constant { value: i64 },
    Node { node: Rc<RefCell<Node>> },
}

#[derive(Clone)]
pub enum Node {
    Start,
    Scope,
    Return {
        ctrl: Rc<RefCell<Node>>,
        node: Rc<RefCell<Node>>,
    },
    Constant {
        value: i64,
    },
    Add {
        ctrl: Rc<RefCell<Node>>,
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
    },
    Sub {
        ctrl: Rc<RefCell<Node>>,
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
    },
    Mul {
        ctrl: Rc<RefCell<Node>>,
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
    },
    Div {
        ctrl: Rc<RefCell<Node>>,
        lhs: Rc<RefCell<Node>>,
        rhs: Rc<RefCell<Node>>,
    },
    Neg {
        ctrl: Rc<RefCell<Node>>,
        node: Rc<RefCell<Node>>,
    },
}

impl Node {
    pub fn optimize(&self) -> Node {
        match self {
            Node::Add { lhs, rhs, .. } => {
                let lhs = lhs.borrow().optimize();
                let rhs = rhs.borrow().optimize();
                match (lhs, rhs) {
                    (Node::Constant { value: lhs }, Node::Constant { value: rhs }) => {
                        Node::Constant { value: lhs + rhs }
                    }
                    _ => self.clone(),
                }
            }
            Node::Sub { lhs, rhs, .. } => {
                let lhs = lhs.borrow().optimize();
                let rhs = rhs.borrow().optimize();
                match (lhs, rhs) {
                    (Node::Constant { value: lhs }, Node::Constant { value: rhs }) => {
                        Node::Constant { value: lhs - rhs }
                    }
                    _ => self.clone(),
                }
            }
            Node::Mul { lhs, rhs, .. } => {
                let lhs = lhs.borrow().optimize();
                let rhs = rhs.borrow().optimize();
                match (lhs, rhs) {
                    (Node::Constant { value: lhs }, Node::Constant { value: rhs }) => {
                        Node::Constant { value: lhs * rhs }
                    }
                    _ => self.clone(),
                }
            }
            Node::Div { lhs, rhs, .. } => {
                let lhs = lhs.borrow().optimize();
                let rhs = rhs.borrow().optimize();
                match (lhs, rhs) {
                    (Node::Constant { value: lhs }, Node::Constant { value: rhs }) => {
                        Node::Constant { value: lhs / rhs }
                    }
                    _ => self.clone(),
                }
            }
            Node::Neg { node, .. } => {
                let node = node.borrow().optimize();
                match node {
                    Node::Constant { value } => Node::Constant { value: -value },
                    _ => self.clone(),
                }
            }
            _ => self.clone(),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Start => write!(f, "Start"),
            Node::Scope => write!(f, "Scope"),
            Node::Return { ctrl, node } => {
                write!(
                    f,
                    "{:016x}: Return({})",
                    ctrl.as_ptr() as u64,
                    node.borrow()
                )
            }
            Node::Constant { value } => write!(f, "Constant({})", value),
            Node::Add { lhs, rhs, .. } => {
                write!(f, "Add({},{})", lhs.borrow(), rhs.borrow())
            }
            Node::Sub { lhs, rhs, .. } => {
                write!(f, "Sub({},{})", lhs.borrow(), rhs.borrow())
            }
            Node::Mul { lhs, rhs, .. } => {
                write!(f, "Mul({},{})", lhs.borrow(), rhs.borrow())
            }
            Node::Div { lhs, rhs, .. } => {
                write!(f, "Div({},{})", lhs.borrow(), rhs.borrow())
            }
            Node::Neg { node, .. } => {
                write!(f, "Neg({})", node.borrow())
            }
        }
    }
}
