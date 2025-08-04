use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Return {
    ctrl: Rc<RefCell<dyn Ctrl>>,
    expr: Rc<RefCell<dyn Expr>>,
}

impl Return {
    pub fn new(ctrl: Rc<RefCell<dyn Ctrl>>, expr: Rc<RefCell<dyn Expr>>) -> Self {
        Self { ctrl, expr }
    }
}

impl Ctrl for Return {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn visualize(
        &mut self,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    ) {
        add_attr(attributes, "label", "\"Return\"");
        let ctrl_id = visualizer.add_ctrl(&self.ctrl);
        visualizer.add_n2n(&gen_id, &ctrl_id, true);
        let expr_id = visualizer.add_expr(&self.expr);
        visualizer.add_n2n(&gen_id, &expr_id, false);
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return({})", self.expr.borrow())
    }
}
