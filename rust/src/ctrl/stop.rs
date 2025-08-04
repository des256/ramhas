use {
    crate::*,
    std::{cell::RefCell, fmt::Display, rc::Rc},
};

pub struct Stop {
    ctrls: Vec<Rc<RefCell<dyn Ctrl>>>,
}

impl Stop {
    pub fn new() -> Self {
        Self { ctrls: vec![] }
    }
}

impl Ctrl for Stop {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn visualize(
        &mut self,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    ) {
        add_attr(attributes, "label", "\"Stop\"");
        for ctrl in self.ctrls.iter() {
            let ctrl_id = visualizer.add_ctrl(ctrl);
            visualizer.add_n2n(&gen_id, &ctrl_id, true);
        }
    }
}

impl Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stop({})", self.ctrls.len())
    }
}
