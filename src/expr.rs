use {
    crate::*,
    graphviz_rust::dot_structures::Attribute,
    std::{fmt::Display, rc::Rc},
};

pub enum Expr {
    Phi {
        ctrl: Rc<Ctrl>,
        exprs: Vec<Rc<Expr>>,
    },
    Constant {
        value: i64,
    },
    Add {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Subtract {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Multiply {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Divide {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Negate {
        expr: Rc<Expr>,
    },
    Modulo {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Not {
        expr: Rc<Expr>,
    },
    And {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Or {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Xor {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    LogicalAnd {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    LogicalOr {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    LogicalNot {
        expr: Rc<Expr>,
    },
    ShiftLeft {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    ShiftRight {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    Equal {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    NotEqual {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    LessThan {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    LessThanOrEqual {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    GreaterThan {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    GreaterThanOrEqual {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
}

impl Expr {
    pub fn peephole(self: Rc<Self>) -> Rc<Expr> {
        match self.as_ref() {
            Expr::Add { lhs, rhs } => {
                let lhs = Rc::clone(&lhs).peephole();
                let rhs = Rc::clone(&rhs).peephole();
                match (lhs.as_ref(), rhs.as_ref()) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        Rc::new(Expr::Constant { value: lhs + rhs })
                    }
                    // TODO: 0 + expr -> expr
                    // TODO: expr + 0 -> expr
                    // TODO: const + expr -> expr + const
                    // TODO: const1 + (expr + const2) -> expr + (const1 + const2)
                    // TODO: (expr1 + const) + expr2 -> (expr1 + expr2) + const
                    // TODO: expr + expr -> expr * 2
                    _ => self,
                }
            }
            Expr::Subtract { lhs, rhs } => {
                let lhs = Rc::clone(&lhs).peephole();
                let rhs = Rc::clone(&rhs).peephole();
                match (lhs.as_ref(), rhs.as_ref()) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        Rc::new(Expr::Constant { value: lhs - rhs })
                    }
                    // TODO: 0 - expr -> -expr
                    // TODO: expr - 0 -> expr
                    // TODO: expr - expr -> 0
                    _ => self,
                }
            }
            Expr::Multiply { lhs, rhs } => {
                let lhs = Rc::clone(&lhs).peephole();
                let rhs = Rc::clone(&rhs).peephole();
                match (lhs.as_ref(), rhs.as_ref()) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        Rc::new(Expr::Constant { value: lhs * rhs })
                    }
                    // TODO: 1 * expr -> expr
                    // TODO: expr * 1 -> expr
                    // TODO: const * expr -> expr * const
                    _ => self,
                }
            }
            Expr::Divide { lhs, rhs } => {
                let lhs = Rc::clone(&lhs).peephole();
                let rhs = Rc::clone(&rhs).peephole();
                match (lhs.as_ref(), rhs.as_ref()) {
                    (Expr::Constant { value: lhs, .. }, Expr::Constant { value: rhs, .. }) => {
                        Rc::new(Expr::Constant { value: lhs / rhs })
                    }
                    // TODO: expr / 1 -> expr
                    _ => self,
                }
            }
            Expr::Negate { expr } => {
                let expr = Rc::clone(&expr).peephole();
                match expr.as_ref() {
                    Expr::Constant { value } => Rc::new(Expr::Constant { value: -value }),
                    _ => self,
                }
            }
            _ => self,
        }
    }

    pub fn visualize(
        &self,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    ) {
        match self {
            Expr::Phi { ctrl, exprs } => {
                add_attr(attributes, "label", "\"Phi\"");
                let ctrl_id = visualizer.add_ctrl(ctrl);
                for expr in exprs.iter() {
                    let expr_id = visualizer.add_expr(expr);
                    visualizer.add_n2n(&gen_id, &expr_id, false);
                }
                visualizer.add_n2n(&gen_id, &ctrl_id, true);
            }
            Expr::Constant { value } => {
                add_attr(attributes, "label", &format!("{}", value));
            }
            Expr::Add { lhs, rhs } => {
                add_attr(attributes, "label", "\"+\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Subtract { lhs, rhs } => {
                add_attr(attributes, "label", "\"-\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Multiply { lhs, rhs } => {
                add_attr(attributes, "label", "\"*\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Divide { lhs, rhs } => {
                add_attr(attributes, "label", "\"/\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Modulo { lhs, rhs } => {
                add_attr(attributes, "label", "\"%\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Negate { expr } => {
                add_attr(attributes, "label", "\"-\"");
                let expr_id = visualizer.add_expr(expr);
                visualizer.add_n2n(&gen_id, &expr_id, false);
            }
            Expr::Not { expr } => {
                add_attr(attributes, "label", "\"~\"");
                let expr_id = visualizer.add_expr(expr);
                visualizer.add_n2n(&gen_id, &expr_id, false);
            }
            Expr::And { lhs, rhs } => {
                add_attr(attributes, "label", "\"&\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Or { lhs, rhs } => {
                add_attr(attributes, "label", "\"|\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Xor { lhs, rhs } => {
                add_attr(attributes, "label", "\"^\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::LogicalAnd { lhs, rhs } => {
                add_attr(attributes, "label", "\"&&\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::LogicalOr { lhs, rhs } => {
                add_attr(attributes, "label", "\"||\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::LogicalNot { expr } => {
                add_attr(attributes, "label", "\"-\"");
                let expr_id = visualizer.add_expr(expr);
                visualizer.add_n2n(&gen_id, &expr_id, false);
            }
            Expr::ShiftLeft { lhs, rhs } => {
                add_attr(attributes, "label", "\"<<\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::ShiftRight { lhs, rhs } => {
                add_attr(attributes, "label", "\">>\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Equal { lhs, rhs } => {
                add_attr(attributes, "label", "\"==\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::NotEqual { lhs, rhs } => {
                add_attr(attributes, "label", "\"!=\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::LessThan { lhs, rhs } => {
                add_attr(attributes, "label", "\"<\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::LessThanOrEqual { lhs, rhs } => {
                add_attr(attributes, "label", "\"<=\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::GreaterThan { lhs, rhs } => {
                add_attr(attributes, "label", "\">\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::GreaterThanOrEqual { lhs, rhs } => {
                add_attr(attributes, "label", "\">=\"");
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Phi { exprs, .. } => write!(f, "Phi({})", exprs.len()),
            Expr::Constant { value } => write!(f, "{}", value),
            Expr::Add { lhs, rhs } => write!(f, "({} + {})", lhs, rhs),
            Expr::Subtract { lhs, rhs } => write!(f, "({} - {})", lhs, rhs),
            Expr::Multiply { lhs, rhs } => write!(f, "({} * {})", lhs, rhs),
            Expr::Divide { lhs, rhs } => write!(f, "({} / {})", lhs, rhs),
            Expr::Negate { expr } => write!(f, "(-{})", expr),
            Expr::Modulo { lhs, rhs } => write!(f, "({} % {})", lhs, rhs),
            Expr::Not { expr } => write!(f, "(!{})", expr),
            Expr::And { lhs, rhs } => write!(f, "({} & {})", lhs, rhs),
            Expr::Or { lhs, rhs } => write!(f, "({} | {})", lhs, rhs),
            Expr::Xor { lhs, rhs } => write!(f, "({} ^ {})", lhs, rhs),
            Expr::LogicalAnd { lhs, rhs } => write!(f, "({} && {})", lhs, rhs),
            Expr::LogicalOr { lhs, rhs } => write!(f, "({} || {})", lhs, rhs),
            Expr::LogicalNot { expr } => write!(f, "(!{})", expr),
            Expr::ShiftLeft { lhs, rhs } => write!(f, "({} << {})", lhs, rhs),
            Expr::ShiftRight { lhs, rhs } => write!(f, "({} >> {})", lhs, rhs),
            Expr::Equal { lhs, rhs } => write!(f, "({} == {})", lhs, rhs),
            Expr::NotEqual { lhs, rhs } => write!(f, "({} != {})", lhs, rhs),
            Expr::LessThan { lhs, rhs } => write!(f, "({} < {})", lhs, rhs),
            Expr::LessThanOrEqual { lhs, rhs } => write!(f, "({} <= {})", lhs, rhs),
            Expr::GreaterThan { lhs, rhs } => write!(f, "({} > {})", lhs, rhs),
            Expr::GreaterThanOrEqual { lhs, rhs } => write!(f, "({} >= {})", lhs, rhs),
        }
    }
}
