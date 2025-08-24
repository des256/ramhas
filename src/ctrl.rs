use crate::*;

#[derive(Debug, Clone)]

pub enum Ctrl {
    Start {
        arg_ids: Vec<Id<Expr>>,
        symbols: Symbols,
    },
    Return {
        ctrl_id: Id<Ctrl>,
        expr_id: Id<Expr>,
    },
    Stop {
        ctrl_ids: Vec<Id<Ctrl>>,
    },
    If {
        ctrl_id: Id<Ctrl>,
        expr_id: Id<Expr>,
        then_id: Id<Ctrl>,
        else_id: Option<Id<Ctrl>>,
    },
    Then {
        ctrl_id: Id<Ctrl>,
        symbols: Symbols,
    },
    Else {
        ctrl_id: Id<Ctrl>,
        symbols: Symbols,
    },
    Merge {
        ctrl_ids: Vec<Id<Ctrl>>,
    },
}

impl Arena<Ctrl> {
    pub fn symbols(&self, id: Id<Ctrl>) -> &Symbols {
        let ctrl = self.get(&id);
        match ctrl {
            Ctrl::Start { symbols, .. } => symbols,
            Ctrl::Then { symbols, .. } => symbols,
            Ctrl::Else { symbols, .. } => symbols,
            _ => panic!("If, Merge and Return have no symbols"),
        }
    }

    pub fn symbols_mut(&mut self, id: Id<Ctrl>) -> &mut Symbols {
        let ctrl = self.get_mut(&id);
        match ctrl {
            Ctrl::Start { symbols, .. } => symbols,
            Ctrl::Then { symbols, .. } => symbols,
            Ctrl::Else { symbols, .. } => symbols,
            _ => panic!("If, Merge and Return have no symbols"),
        }
    }
}

/*
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
*/
