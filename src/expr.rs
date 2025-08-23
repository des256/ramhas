use {
    crate::*,
    graphviz_rust::dot_structures::Attribute,
    std::{fmt::Display, rc::Rc},
};

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    Xor,
    LogicalAnd,
    LogicalOr,
    ShiftLeft,
    ShiftRight,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
            BinaryOp::Modulo => write!(f, "%"),
            BinaryOp::And => write!(f, "&"),
            BinaryOp::Or => write!(f, "|"),
            BinaryOp::Xor => write!(f, "^"),
            BinaryOp::LogicalAnd => write!(f, "&&"),
            BinaryOp::LogicalOr => write!(f, "||"),
            BinaryOp::ShiftLeft => write!(f, "<<"),
            BinaryOp::ShiftRight => write!(f, ">>"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::LessThan => write!(f, "<"),
            BinaryOp::GreaterThan => write!(f, ">"),
            BinaryOp::LessThanOrEqual => write!(f, "<="),
            BinaryOp::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

pub enum UnaryOp {
    Negate,
    Not,
    LogicalNot,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
            UnaryOp::Not => write!(f, "~"),
            UnaryOp::LogicalNot => write!(f, "!"),
        }
    }
}

pub enum Expr {
    Phi {
        ctrl: Rc<Ctrl>,
        exprs: Vec<Rc<Expr>>,
    },
    Constant {
        value: Value,
    },
    Binary {
        lhs: Rc<Expr>,
        op: BinaryOp,
        rhs: Rc<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Rc<Expr>,
    },
}

impl Expr {
    pub fn peephole(self: Rc<Self>) -> Rc<Expr> {
        let value = self.compute();
        match value {
            Value::Int(IntValue::Constant(_)) => Rc::new(Expr::Constant { value }),
            Value::Bool(BoolValue::Constant(_)) => Rc::new(Expr::Constant { value }),
            _ => self,
        }
    }

    pub fn compute(&self) -> Value {
        match self {
            Expr::Phi { exprs, .. } => {
                let mut value = Value::Any;
                for expr in exprs.iter() {
                    value = value.join(&expr.compute());
                }
                value
            }
            Expr::Constant { value } => value.clone(),
            Expr::Binary { lhs, op, rhs } => {
                let lhs = lhs.compute();
                let rhs = rhs.compute();
                if let (
                    &Value::Int(IntValue::Constant(lhs)),
                    &Value::Int(IntValue::Constant(rhs)),
                ) = (&lhs, &rhs)
                {
                    match op {
                        BinaryOp::Add => Value::Int(IntValue::Constant(lhs + rhs)),
                        BinaryOp::Subtract => Value::Int(IntValue::Constant(lhs - rhs)),
                        BinaryOp::Multiply => Value::Int(IntValue::Constant(lhs * rhs)),
                        BinaryOp::Divide => Value::Int(IntValue::Constant(lhs / rhs)),
                        BinaryOp::Modulo => Value::Int(IntValue::Constant(lhs % rhs)),
                        BinaryOp::And => Value::Int(IntValue::Constant(lhs & rhs)),
                        BinaryOp::Or => Value::Int(IntValue::Constant(lhs | rhs)),
                        BinaryOp::Xor => Value::Int(IntValue::Constant(lhs ^ rhs)),
                        BinaryOp::ShiftLeft => Value::Int(IntValue::Constant(lhs << rhs)),
                        BinaryOp::ShiftRight => Value::Int(IntValue::Constant(lhs >> rhs)),
                        BinaryOp::Equal => Value::Bool(BoolValue::Constant(lhs == rhs)),
                        BinaryOp::NotEqual => Value::Bool(BoolValue::Constant(lhs != rhs)),
                        BinaryOp::LessThan => Value::Bool(BoolValue::Constant(lhs < rhs)),
                        BinaryOp::GreaterThan => Value::Bool(BoolValue::Constant(lhs > rhs)),
                        BinaryOp::LessThanOrEqual => Value::Bool(BoolValue::Constant(lhs <= rhs)),
                        BinaryOp::GreaterThanOrEqual => {
                            Value::Bool(BoolValue::Constant(lhs >= rhs))
                        }
                        _ => panic!("binary operator '{}' invalid for integers", op),
                    }
                } else if let (
                    &Value::Bool(BoolValue::Constant(lhs)),
                    &Value::Bool(BoolValue::Constant(rhs)),
                ) = (&lhs, &rhs)
                {
                    match op {
                        BinaryOp::LogicalAnd => Value::Bool(BoolValue::Constant(lhs && rhs)),
                        BinaryOp::LogicalOr => Value::Bool(BoolValue::Constant(lhs || rhs)),
                        _ => panic!("binary operator '{}' invalid for booleans", op),
                    }
                } else {
                    lhs.join(&rhs)
                }
            }
            Expr::Unary { op, expr } => {
                let expr = expr.compute();
                if let Value::Int(IntValue::Constant(expr)) = expr {
                    match op {
                        UnaryOp::Negate => Value::Int(IntValue::Constant(-expr)),
                        UnaryOp::Not => Value::Int(IntValue::Constant(!expr)),
                        _ => panic!("unary operator '{}' invalid for integers", op),
                    }
                } else if let Value::Bool(BoolValue::Constant(expr)) = expr {
                    match op {
                        UnaryOp::LogicalNot => Value::Bool(BoolValue::Constant(!expr)),
                        _ => panic!("unary operator '{}' invalid for booleans", op),
                    }
                } else {
                    expr
                }
            }
        }
    }

    pub fn idealize(self: Rc<Self>) -> Rc<Self> {
        // TODO: 0 + expr -> expr
        // TODO: expr + 0 -> expr
        // TODO: const + expr -> expr + const
        // TODO: const1 + (expr + const2) -> expr + (const1 + const2)
        // TODO: (expr1 + const) + expr2 -> (expr1 + expr2) + const
        // TODO: expr + expr -> expr * 2
        // TODO: 0 - expr -> -expr
        // TODO: expr - 0 -> expr
        // TODO: expr - expr -> 0
        // TODO: 1 * expr -> expr
        // TODO: expr * 1 -> expr
        // TODO: const * expr -> expr * const
        // TODO: expr / 1 -> expr
        // TODO: expr == expr -> true
        // TODO: expr != expr -> false
        // TODO: expr < expr -> false
        // TODO: expr > expr -> false
        // TODO: expr <= expr -> true
        // TODO: expr >= expr -> true
        self
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
            Expr::Binary { lhs, op, rhs } => {
                add_attr(attributes, "label", &format!("\"{}\"", op));
                let lhs_id = visualizer.add_expr(lhs);
                let rhs_id = visualizer.add_expr(rhs);
                visualizer.add_n2n(&gen_id, &lhs_id, false);
                visualizer.add_n2n(&gen_id, &rhs_id, false);
            }
            Expr::Unary { op, expr } => {
                add_attr(attributes, "label", &format!("\"{}\"", op));
                let expr_id = visualizer.add_expr(expr);
                visualizer.add_n2n(&gen_id, &expr_id, false);
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Phi { exprs, .. } => write!(f, "Phi({})", exprs.len()),
            Expr::Constant { value } => write!(f, "{}", value),
            Expr::Binary { lhs, op, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::Unary { op, expr } => write!(f, "({} {})", op, expr),
        }
    }
}
