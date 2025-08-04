use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Start {
    args: Vec<Rc<RefCell<dyn Expr>>>,
    scopes: Scopes,
}

impl Start {
    pub fn new() -> Self {
        Self {
            args: vec![],
            scopes: Scopes::new(),
        }
    }
}

impl Ctrl for Start {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn visualize(
        &mut self,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    ) {
        let mut label = "\"{Start|{args".to_string();
        for (i, arg) in self.args.iter().enumerate() {
            let expr_id = visualizer.add_expr(arg);
            label.push_str(&format!("|<arg{}>{}", i, expr_id));
            visualizer.add_p2n(&gen_id, &format!("arg{}", i), &expr_id, false);
        }
        let scopes_id = visualizer.add_scopes(&gen_id, &self.scopes);
        label.push_str("}}\"");
        visualizer.add_n2n(&gen_id, &scopes_id, false);
        add_attr(attributes, "label", &label);
    }
}

impl Symbols for Start {
    fn declare(&mut self, name: &str, expr: Rc<RefCell<dyn Expr>>) {
        self.scopes.declare(name, expr);
    }

    fn set(&mut self, name: &str, expr: Rc<RefCell<dyn Expr>>) {
        self.scopes.set(name, expr);
    }

    fn get(&self, name: &str) -> Option<Rc<RefCell<dyn Expr>>> {
        self.scopes.get(name)
    }

    fn push_scope(&mut self) {
        self.scopes.push();
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

impl Display for Start {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Start({})", self.args.len())
    }
}
