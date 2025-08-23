use {
    crate::*,
    graphviz_rust::dot_structures::Attribute,
    std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc},
};

#[derive(Debug, Clone)]

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
        then: Rc<Ctrl>,
        r#else: Option<Rc<Ctrl>>,
    },
    Then {
        ctrl: Rc<Ctrl>,
        bindings: Rc<RefCell<Vec<HashMap<String, Rc<Expr>>>>>,
    },
    Else {
        ctrl: Rc<Ctrl>,
        bindings: Rc<RefCell<Vec<HashMap<String, Rc<Expr>>>>>,
    },
    Merge {
        ctrls: Vec<Rc<Ctrl>>,
    },
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

    pub fn new_if(
        ctrl: Rc<Ctrl>,
        expr: Rc<Expr>,
        then: Rc<Ctrl>,
        r#else: Option<Rc<Ctrl>>,
    ) -> Rc<Ctrl> {
        Rc::new(Ctrl::If {
            ctrl,
            expr,
            then,
            r#else,
        })
    }

    pub fn new_then(ctrl: Rc<Ctrl>) -> Rc<Ctrl> {
        let bindings = ctrl.clone_bindings();
        Rc::new(Ctrl::Then { ctrl, bindings })
    }

    pub fn new_else(ctrl: Rc<Ctrl>) -> Rc<Ctrl> {
        let bindings = ctrl.clone_bindings();
        Rc::new(Ctrl::Else { ctrl, bindings })
    }

    pub fn new_merge(ctrls: Vec<Rc<Ctrl>>) -> Rc<Ctrl> {
        Rc::new(Ctrl::Merge { ctrls })
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
            Ctrl::If {
                ctrl,
                expr,
                then,
                r#else,
            } => {
                let ctrl_id = visualizer.add_ctrl(&ctrl);
                let expr_id = visualizer.add_expr(&expr);
                let then_id = visualizer.add_ctrl(&then);
                let r#else_id = if let Some(r#else) = r#else {
                    Some(visualizer.add_ctrl(&r#else))
                } else {
                    None
                };
                add_attr(attributes, "label", "\"If\"");
                visualizer.add_n2n(&gen_id, &ctrl_id, false);
                visualizer.add_n2n(&gen_id, &expr_id, false);
                visualizer.add_n2n(&gen_id, &then_id, true);
                if let Some(r#else_id) = r#else_id {
                    visualizer.add_n2n(&gen_id, &r#else_id, true);
                }
            }
            Ctrl::Then { ctrl, bindings } => {
                let ctrl_id = visualizer.add_ctrl(&ctrl);
                let scopes_id = visualizer.add_bindings(&gen_id, &bindings);
                add_attr(attributes, "label", "\"Then\"");
                visualizer.add_n2n(&gen_id, &ctrl_id, false);
                visualizer.add_n2n(&gen_id, &scopes_id, false);
            }
            Ctrl::Else { ctrl, bindings } => {
                let ctrl_id = visualizer.add_ctrl(&ctrl);
                let scopes_id = visualizer.add_bindings(&gen_id, &bindings);
                add_attr(attributes, "label", "\"Else\"");
                visualizer.add_n2n(&gen_id, &ctrl_id, false);
                visualizer.add_n2n(&gen_id, &scopes_id, false);
                visualizer.add_n2n(&gen_id, &ctrl_id, true);
            }
            Ctrl::Merge { ctrls } => {
                for ctrl in ctrls.iter() {
                    let ctrl_id = visualizer.add_ctrl(ctrl);
                    visualizer.add_n2n(&gen_id, &ctrl_id, true);
                }
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
            Ctrl::Then { bindings, .. } => {
                for bindings in bindings.borrow().iter().rev() {
                    if let Some(expr) = bindings.get(name) {
                        return Some(Rc::clone(&expr));
                    }
                }
                None
            }
            Ctrl::Else { bindings, .. } => {
                for bindings in bindings.borrow().iter().rev() {
                    if let Some(expr) = bindings.get(name) {
                        return Some(Rc::clone(&expr));
                    }
                }
                None
            }
            _ => {
                panic!("cannot call get on {:?}", self);
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
            Ctrl::Then { bindings, .. } => {
                for bindings in bindings.borrow_mut().iter_mut().rev() {
                    if let Some(binding) = bindings.get_mut(name) {
                        *binding = expr;
                        return;
                    }
                }
            }
            Ctrl::Else { bindings, .. } => {
                for bindings in bindings.borrow_mut().iter_mut().rev() {
                    if let Some(binding) = bindings.get_mut(name) {
                        *binding = expr;
                        return;
                    }
                }
            }
            _ => {
                panic!("cannot call set on {:?}", self);
            }
        }
    }

    fn clone_bindings(&self) -> Rc<RefCell<Vec<HashMap<String, Rc<Expr>>>>> {
        match self {
            Ctrl::Start { bindings, .. } => Rc::clone(bindings),
            Ctrl::Then { bindings, .. } => Rc::clone(bindings),
            Ctrl::Else { bindings, .. } => Rc::clone(bindings),
            _ => panic!("cannot copy bindings from {:?}", self),
        }
    }
}
