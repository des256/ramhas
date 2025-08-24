use crate::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Phi {
        ctrl: Id<Ctrl>,
        expr_ids: Vec<Id<Expr>>,
    },
    Constant {
        value: Value,
    },
    Binary {
        lhs_id: Id<Expr>,
        op: BinaryOp,
        rhs_id: Id<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr_id: Id<Expr>,
    },
}

impl Arena<Expr> {
    pub fn peephole(&mut self, id: Id<Expr>) -> Id<Expr> {
        let value = self.compute(id);
        match value {
            Value::All => {}
            Value::Int(value) => {
                // if integer constant, solidify
                if let IntValue::Constant(_) = value {
                    return self.alloc(Expr::Constant {
                        value: Value::Int(value),
                    });
                }
                let expr = self.get(&id);
                match expr {
                    Expr::Phi { .. } => {}
                    Expr::Constant { .. } => {}
                    Expr::Binary { lhs_id, op, rhs_id } => {
                        let lhs = self.get(&lhs_id);
                        let rhs = self.get(&rhs_id);
                        match op {
                            BinaryOp::Add => {
                                // 0 + expr -> expr
                                // const + expr -> expr + const
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(lhs_value)),
                                } = lhs
                                {
                                    if *lhs_value == 0 {
                                        return *rhs_id;
                                    } else {
                                        return self.alloc(Expr::Binary {
                                            lhs_id: *rhs_id,
                                            op: BinaryOp::Add,
                                            rhs_id: *lhs_id,
                                        });
                                    }
                                }
                                // expr + 0 -> expr
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(0)),
                                } = rhs
                                {
                                    return *lhs_id;
                                }
                                // TODO: const1 + (expr + const2) -> expr + (const1 + const2)
                                // TODO: (expr1 + const) + expr2 -> (expr1 + expr2) + const
                                // TODO: expr + expr -> expr * 2
                            }
                            BinaryOp::Subtract => {
                                // 0 - expr -> -expr
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(0)),
                                } = lhs
                                {
                                    return self.alloc(Expr::Unary {
                                        op: UnaryOp::Negate,
                                        expr_id: *rhs_id,
                                    });
                                }
                                // expr - 0 -> expr
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(0)),
                                } = rhs
                                {
                                    return *lhs_id;
                                }
                                // expr - expr -> 0
                                if *lhs_id == *rhs_id {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Int(IntValue::Constant(0)),
                                    });
                                }
                            }
                            BinaryOp::Multiply => {
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(lhs_value)),
                                } = lhs
                                {
                                    // 0 * expr -> 0
                                    if *lhs_value == 0 {
                                        return self.alloc(Expr::Constant {
                                            value: Value::Int(IntValue::Constant(0)),
                                        });
                                    }
                                    // 1 * expr -> expr
                                    if *lhs_value == 1 {
                                        return *rhs_id;
                                    }
                                    // -1 * expr -> -expr
                                    if *lhs_value == -1 {
                                        return self.alloc(Expr::Unary {
                                            op: UnaryOp::Negate,
                                            expr_id: *rhs_id,
                                        });
                                    }
                                    // const * expr -> expr * const
                                    else {
                                        return self.alloc(Expr::Binary {
                                            lhs_id: *rhs_id,
                                            op: BinaryOp::Multiply,
                                            rhs_id: *lhs_id,
                                        });
                                    }
                                }
                            }
                            BinaryOp::Divide => {
                                // 0 / expr -> 0
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(0)),
                                } = lhs
                                {
                                    return *lhs_id;
                                }
                                // expr / 1 -> expr
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(1)),
                                } = rhs
                                {
                                    return *lhs_id;
                                }
                            }
                            BinaryOp::Modulo => {
                                // 0 % expr -> 0
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(0)),
                                } = lhs
                                {
                                    return *lhs_id;
                                }
                                // expr % 1 -> 0
                                if let Expr::Constant {
                                    value: Value::Int(IntValue::Constant(1)),
                                } = rhs
                                {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Int(IntValue::Constant(0)),
                                    });
                                }
                            }
                            BinaryOp::LogicalAnd => {}
                            BinaryOp::LogicalOr => {}
                            BinaryOp::And => {}
                            BinaryOp::Or => {}
                            BinaryOp::Xor => {}
                            BinaryOp::ShiftLeft => {}
                            BinaryOp::ShiftRight => {}
                            BinaryOp::Equal => {}
                            BinaryOp::NotEqual => {}
                            BinaryOp::LessThan => {}
                            BinaryOp::GreaterThan => {}
                            BinaryOp::LessThanOrEqual => {}
                            BinaryOp::GreaterThanOrEqual => {}
                        }
                    }
                    Expr::Unary { .. } => {}
                }
            }
            Value::Bool(value) => {
                if let BoolValue::Constant(_) = value {
                    return self.alloc(Expr::Constant {
                        value: Value::Bool(value),
                    });
                }
                let expr = self.get(&id);
                match expr {
                    Expr::Phi { .. } => {}
                    Expr::Constant { .. } => {}
                    Expr::Binary { lhs_id, op, rhs_id } => {
                        let lhs = self.get(&lhs_id);
                        let rhs = self.get(&rhs_id);
                        match op {
                            BinaryOp::LogicalAnd => {
                                // false && expr -> false
                                if let Expr::Constant {
                                    value: Value::Bool(BoolValue::Constant(false)),
                                } = lhs
                                {
                                    return *lhs_id;
                                }
                                // expr && false -> false
                                if let Expr::Constant {
                                    value: Value::Bool(BoolValue::Constant(false)),
                                } = rhs
                                {
                                    return *rhs_id;
                                }
                            }
                            BinaryOp::LogicalOr => {
                                // true || expr -> true
                                if let Expr::Constant {
                                    value: Value::Bool(BoolValue::Constant(true)),
                                } = lhs
                                {
                                    return *lhs_id;
                                }
                                // expr || true -> true
                                if let Expr::Constant {
                                    value: Value::Bool(BoolValue::Constant(true)),
                                } = rhs
                                {
                                    return *rhs_id;
                                }
                            }
                            BinaryOp::Equal => {
                                // expr == expr -> true
                                if lhs_id == rhs_id {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(true)),
                                    });
                                }
                            }
                            BinaryOp::NotEqual => {
                                // expr != expr -> false
                                if lhs_id == rhs_id {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(false)),
                                    });
                                }
                            }
                            BinaryOp::LessThan => {
                                // expr < expr -> false
                                if lhs_id == rhs_id {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(false)),
                                    });
                                }
                            }
                            BinaryOp::GreaterThan => {
                                // expr > expr -> false
                                if lhs_id == rhs_id {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(false)),
                                    });
                                }
                            }
                            BinaryOp::LessThanOrEqual => {
                                // expr <= expr -> true
                                if lhs_id == rhs_id {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(true)),
                                    });
                                }
                            }
                            BinaryOp::GreaterThanOrEqual => {
                                // expr >= expr -> true
                                if lhs_id == rhs_id {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(true)),
                                    });
                                }
                            }
                            BinaryOp::Add => {}
                            BinaryOp::Subtract => {}
                            BinaryOp::Multiply => {}
                            BinaryOp::Divide => {}
                            BinaryOp::Modulo => {}
                            BinaryOp::And => {}
                            BinaryOp::Or => {}
                            BinaryOp::Xor => {}
                            BinaryOp::ShiftLeft => {}
                            BinaryOp::ShiftRight => {}
                        }
                    }
                    Expr::Unary { op, expr_id } => {
                        let expr = self.get(&expr_id);
                        match op {
                            UnaryOp::Not => {
                                // !true -> false
                                if let Expr::Constant {
                                    value: Value::Bool(BoolValue::Constant(true)),
                                } = expr
                                {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(false)),
                                    });
                                }
                                // !false -> true
                                if let Expr::Constant {
                                    value: Value::Bool(BoolValue::Constant(false)),
                                } = expr
                                {
                                    return self.alloc(Expr::Constant {
                                        value: Value::Bool(BoolValue::Constant(true)),
                                    });
                                }
                            }
                            UnaryOp::Negate => {}
                        }
                    }
                }
            }
            Value::Any => {}
        }
        id
    }

    pub fn compute(&self, expr_id: Id<Expr>) -> Value {
        let expr = self.get(&expr_id);
        match expr {
            Expr::Phi { expr_ids, .. } => {
                let mut value = Value::Any;
                for id in expr_ids.iter() {
                    value = value.join(&self.compute(*id));
                }
                value
            }
            Expr::Constant { value } => value.clone(),
            Expr::Binary { lhs_id, op, rhs_id } => {
                let lhs = self.compute(*lhs_id);
                let rhs = self.compute(*rhs_id);
                if let (
                    &Value::Int(IntValue::Constant(lhs_value)),
                    &Value::Int(IntValue::Constant(rhs_value)),
                ) = (&lhs, &rhs)
                {
                    match op {
                        BinaryOp::Add => Value::Int(IntValue::Constant(lhs_value + rhs_value)),
                        BinaryOp::Subtract => Value::Int(IntValue::Constant(lhs_value - rhs_value)),
                        BinaryOp::Multiply => Value::Int(IntValue::Constant(lhs_value * rhs_value)),
                        BinaryOp::Divide => Value::Int(IntValue::Constant(lhs_value / rhs_value)),
                        BinaryOp::Modulo => Value::Int(IntValue::Constant(lhs_value % rhs_value)),
                        BinaryOp::And => Value::Int(IntValue::Constant(lhs_value & rhs_value)),
                        BinaryOp::Or => Value::Int(IntValue::Constant(lhs_value | rhs_value)),
                        BinaryOp::Xor => Value::Int(IntValue::Constant(lhs_value ^ rhs_value)),
                        BinaryOp::ShiftLeft => {
                            Value::Int(IntValue::Constant(lhs_value << rhs_value))
                        }
                        BinaryOp::ShiftRight => {
                            Value::Int(IntValue::Constant(lhs_value >> rhs_value))
                        }
                        BinaryOp::Equal => Value::Bool(BoolValue::Constant(lhs_value == rhs_value)),
                        BinaryOp::NotEqual => {
                            Value::Bool(BoolValue::Constant(lhs_value != rhs_value))
                        }
                        BinaryOp::LessThan => {
                            Value::Bool(BoolValue::Constant(lhs_value < rhs_value))
                        }
                        BinaryOp::GreaterThan => {
                            Value::Bool(BoolValue::Constant(lhs_value > rhs_value))
                        }
                        BinaryOp::LessThanOrEqual => {
                            Value::Bool(BoolValue::Constant(lhs_value <= rhs_value))
                        }
                        BinaryOp::GreaterThanOrEqual => {
                            Value::Bool(BoolValue::Constant(lhs_value >= rhs_value))
                        }
                        _ => panic!("binary operator '{}' invalid for integers", op),
                    }
                } else if let (
                    &Value::Bool(BoolValue::Constant(lhs_value)),
                    &Value::Bool(BoolValue::Constant(rhs_value)),
                ) = (&lhs, &rhs)
                {
                    match op {
                        BinaryOp::LogicalAnd => {
                            Value::Bool(BoolValue::Constant(lhs_value && rhs_value))
                        }
                        BinaryOp::LogicalOr => {
                            Value::Bool(BoolValue::Constant(lhs_value || rhs_value))
                        }
                        _ => panic!("binary operator '{}' invalid for booleans", op),
                    }
                } else {
                    lhs.join(&rhs)
                }
            }
            Expr::Unary { op, expr_id } => {
                let expr = self.compute(*expr_id);
                if let Value::Int(IntValue::Constant(expr)) = expr {
                    match op {
                        UnaryOp::Negate => Value::Int(IntValue::Constant(-expr)),
                        UnaryOp::Not => Value::Int(IntValue::Constant(!expr)),
                    }
                } else if let Value::Bool(BoolValue::Constant(expr)) = expr {
                    match op {
                        UnaryOp::Not => Value::Bool(BoolValue::Constant(!expr)),
                        _ => panic!("unary operator '{}' invalid for booleans", op),
                    }
                } else {
                    expr
                }
            }
        }
    }

    /*
    pub fn visualize(
        &self,
        id: Id<Expr>,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    ) {
        let expr = self.get(&id);
        match expr {
            Expr::Phi { ctrl, expr_ids } => {
                add_attr(attributes, "label", "\"Phi\"");
                let ctrl_id = visualizer.add_ctrl(self, ctrl);
                for expr_id in expr_ids.iter() {
                    let expr_id = visualizer.add_expr(self, expr_id);
                    visualizer.add_n2n(&gen_id, &expr_id, false);
                }
                visualizer.add_n2n(&gen_id, &ctrl_id, true);
            }
            Expr::Constant { value } => {
                add_attr(attributes, "label", &format!("{}", value));
            }
            Expr::Binary { lhs_id, op, rhs_id } => {
                add_attr(attributes, "label", &format!("\"{}\"", op));
            }
            Expr::Unary { op, expr_id } => {
                add_attr(attributes, "label", &format!("\"{}\"", op));
            }
        }
    }
    */
}
