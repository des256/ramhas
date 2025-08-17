use {
    crate::*,
    graphviz_rust::dot_structures::Attribute,
    std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc},
};

pub enum Ctrl {
    Start {
        args: Vec<Rc<Expr>>,
        bindings: Rc<RefCell<Vec<HashMap<String, Rc<Expr>>>>>,
    },
    Return {
        ctrl: Rc<Ctrl>,
        expr: Rc<Expr>,
    },
    Stop {
        ctrls: Vec<Rc<Ctrl>>,
    },
    If {
        ctrl: Rc<Ctrl>,
        expr: Rc<Expr>,
        ifthen: Rc<Ctrl>,
        ifelse: Option<Rc<Ctrl>>,
    },
    IfThen {
        ctrl: Rc<Ctrl>,
    },
    IfElse {
        ctrl: Rc<Ctrl>,
    },
    Region {
        ctrl: Rc<Ctrl>,
    }
}

impl Ctrl {
    pub fn new_start(args: Vec<Rc<Expr>>) -> Rc<Ctrl> {
        Rc::new(Ctrl::Start {
            args,
            bindings: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn new_return(ctrl: Rc<Ctrl>, expr: Rc<Expr>) -> Rc<Ctrl> {
        Rc::new(Ctrl::Return { ctrl, expr })
    }

    pub fn new_stop(ctrls: Vec<Rc<Ctrl>>) -> Rc<Ctrl> {
        Rc::new(Ctrl::Stop { ctrls })
    }

    pub fn visualize(
        &self,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    ) {
        match self {
            Ctrl::Start { args, bindings } => {
                let mut label = "\"{Start|{args".to_string();
                for (i, arg) in args.iter().enumerate() {
                    let expr_id = visualizer.add_expr(arg);
                    label.push_str(&format!("|<arg{}>{}", i, expr_id));
                    visualizer.add_p2n(&gen_id, &format!("arg{}", i), &expr_id, false);
                }
                let scopes_id = visualizer.add_bindings(&gen_id, &bindings);
                label.push_str("}}\"");
                visualizer.add_n2n(&gen_id, &scopes_id, false);
                add_attr(attributes, "label", &label);
            }
            Ctrl::Return { ctrl, expr } => {
                add_attr(attributes, "label", "\"Return\"");
                let ctrl_id = visualizer.add_ctrl(&ctrl);
                visualizer.add_n2n(&gen_id, &ctrl_id, true);
                let expr_id = visualizer.add_expr(&expr);
                visualizer.add_n2n(&gen_id, &expr_id, false);
            }
            Ctrl::Stop { ctrls } => {
                add_attr(attributes, "label", "\"Stop\"");
                for ctrl in ctrls.iter() {
                    let ctrl_id = visualizer.add_ctrl(ctrl);
                    visualizer.add_n2n(&gen_id, &ctrl_id, true);
                }
            }
            Ctrl::If { ctrl,expr,ifthen,ifelse } => {

            }
            Ctrl::IfThen { ctrl } => {

            }
            Ctrl::IfElse { ctrl } => {

            }
            ctrl::Region { ctrl } => {

            }
        }
    }

    pub fn push_scope(&self) {
        match self {
            Ctrl::Start { bindings, .. } => {
                bindings.borrow_mut().push(HashMap::new());
            }
            _ => {
                panic!("cannot call push_scope on non-start control");
            }
        }
    }

    pub fn pop_scope(&self) {
        match self {
            Ctrl::Start { bindings, .. } => {
                bindings.borrow_mut().pop();
            }
            _ => {
                panic!("cannot call pop_scope on non-start control");
            }
        }
    }

    pub fn declare(&self, name: &str, expr: Rc<Expr>) {
        match self {
            Ctrl::Start { bindings, .. } => {
                bindings
                    .borrow_mut()
                    .last_mut()
                    .unwrap()
                    .insert(name.to_string(), expr);
            }
            _ => {
                panic!("cannot call declare on non-start control");
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<Expr>> {
        match self {
            Ctrl::Start { bindings, .. } => {
                for bindings in bindings.borrow().iter().rev() {
                    if let Some(expr) = bindings.get(name) {
                        return Some(Rc::clone(&expr));
                    }
                }
                None
            }
            _ => {
                panic!("cannot call get on non-start control");
            }
        }
    }

    pub fn set(&self, name: &str, expr: Rc<Expr>) {
        match self {
            Ctrl::Start { bindings, .. } => {
                for bindings in bindings.borrow_mut().iter_mut().rev() {
                    if let Some(binding) = bindings.get_mut(name) {
                        *binding = expr;
                        return;
                    }
                }
                panic!("variable `{}` not found", name);
            }
            _ => {
                panic!("cannot call set on non-start control");
            }
        }
    }
}

impl Display for Ctrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ctrl::Start { .. } => write!(f, "Start"),
            Ctrl::Return { expr, .. } => write!(f, "Return({})", expr),
            Ctrl::Stop { ctrls } => write!(f, "Stop({})", ctrls.len()),
            Ctrl::If { expr,..} => write!(f,"If({})",expr),
            Ctrl::IfThen { .. } => write!(f,"IfThen"),
            Ctrl::IfElse { .. } => write!(f,"IfElse"),
            Ctrl::Region { .. } => write!(f,"Region"),
        }
    }
}
