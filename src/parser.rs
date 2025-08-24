use crate::*;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: Option<Token>,
    pi: usize,
    exprs: Arena<Expr>,
    ctrls: Arena<Ctrl>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(source);
        let current = tokenizer.next();
        Self {
            tokenizer,
            current,
            pi: 0,
            exprs: Arena::new(),
            ctrls: Arena::new(),
        }
    }

    pub fn print_tokens(&mut self) {
        while let Some(t) = &self.current {
            println!("{}", t);
            self.consume();
        }
    }

    fn consume(&mut self) {
        self.current = self.tokenizer.next();
    }

    fn expect(&mut self, token: Token) {
        if let Some(t) = &self.current {
            if *t == token {
                self.consume();
            } else {
                panic!("expected `{}`, got `{}`", token, t);
            }
        } else {
            panic!("expected `{}`, got end of source", token);
        }
    }

    pub fn parse_program(&mut self) -> Id<Ctrl> {
        let ctrl_id: Id<Ctrl> = self.ctrls.alloc(Ctrl::Start {
            arg_ids: Vec::new(),
            symbols: Symbols::new(),
        });

        self.ctrls.symbols_mut(ctrl_id).push_scope();

        // TODO: add args to scope as Proj
        let mut result: Option<Id<Ctrl>> = None;
        while let Some(_) = self.current {
            result = self.parse_statement(ctrl_id);
        }

        self.ctrls.symbols_mut(ctrl_id).pop_scope();

        if let Some(result) = result {
            result
        } else {
            panic!("program: return statement expected");
        }
    }

    fn parse_statement(&mut self, ctrl_id: Id<Ctrl>) -> Option<Id<Ctrl>> {
        //#[allow(unused_assignments)]
        //let mut title = String::new();
        let result_id: Option<Id<Ctrl>> = match &self.current {
            Some(Token::Return) => {
                self.consume();
                let expr_id = self.parse_expression(ctrl_id);
                let expr_id = self.exprs.peephole(expr_id);
                //title = format!("return {};", expr_id);
                let result_id = self.ctrls.alloc(Ctrl::Return { ctrl_id, expr_id });
                self.expect(Token::Semicolon);
                Some(result_id)
            }
            Some(Token::Int) => {
                self.consume();
                let name = if let Some(Token::Identifier(name)) = &self.current {
                    name.clone()
                } else {
                    panic!("declaration statement: identifier expected");
                };
                self.consume(); // name
                self.expect(Token::Equal);
                let expr_id = self.parse_expression(ctrl_id);
                let expr_id = self.exprs.peephole(expr_id);
                //title = format!("int {} = {};", name, expr_id);
                self.expect(Token::Semicolon);
                self.ctrls.symbols_mut(ctrl_id).declare(&name, expr_id);
                None
            }
            Some(Token::OpenBrace) => {
                self.consume();
                self.ctrls.symbols_mut(ctrl_id).push_scope();
                let mut result_id: Option<Id<Ctrl>> = None;
                loop {
                    match self.current {
                        Some(Token::CloseBrace) => {
                            self.consume();
                            break;
                        }
                        None => {
                            panic!("block statement: unexpected end of source");
                        }
                        _ => {
                            result_id = self.parse_statement(ctrl_id);
                        }
                    }
                }
                self.ctrls.symbols_mut(ctrl_id).pop_scope();
                //title = "{}".to_string();
                result_id
            }
            Some(Token::If) => {
                self.consume(); // if
                let expr_id = self.parse_expression(ctrl_id);
                let expr_id = self.exprs.peephole(expr_id);
                let symbols = self.ctrls.symbols(ctrl_id).clone();
                let then_id = self.ctrls.alloc(Ctrl::Then { ctrl_id, symbols });
                self.parse_statement(then_id);
                let else_id = if let Some(Token::Else) = self.current {
                    self.consume(); // else
                    let symbols = self.ctrls.symbols(ctrl_id).clone();
                    let else_id = self.ctrls.alloc(Ctrl::Else { ctrl_id, symbols });
                    self.parse_statement(else_id);
                    Some(else_id)
                } else {
                    None
                };
                //title = format!("if ({}) {{}}", expr_id);
                let result_id = self.ctrls.alloc(Ctrl::If {
                    ctrl_id,
                    expr_id,
                    then_id,
                    else_id,
                });
                Some(result_id)
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.consume(); // identifier
                self.expect(Token::Equal);
                let expr_id = self.parse_expression(ctrl_id);
                let expr_id = self.exprs.peephole(expr_id);
                //title = format!("{} = {};", name, expr_id);
                self.expect(Token::Semicolon);
                self.ctrls.symbols_mut(ctrl_id).set(&name, expr_id);
                None
            }
            Some(token) => panic!("statement: unexpected `{}`", token),
            None => panic!("statement: unexpected end of source"),
        };
        if let Some(_result_id) = &result_id {
            //let result_id = Rc::clone(&result_id);
            //visualize(&result, &title, Path::new(&format!("test{}.svg", self.pi))).unwrap();
        } else {
            //visualize(&ctrl, &title, Path::new(&format!("test{}.svg", self.pi))).unwrap();
        }
        self.pi += 1;
        if let Some(result_id) = result_id {
            Some(result_id)
        } else {
            None
        }
    }

    fn parse_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        self.parse_logical_or_expression(ctrl_id)
    }

    fn parse_logical_or_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_logical_and_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::BarBar) => {
                    self.consume();
                    let rhs_id = self.parse_logical_and_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::LogicalOr,
                        rhs_id,
                    })
                }
                None => {
                    panic!("logical or expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_logical_and_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_or_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::AmpAmp) => {
                    self.consume();
                    let rhs_id = self.parse_or_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::LogicalAnd,
                        rhs_id,
                    })
                }
                None => {
                    panic!("logical and expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_or_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_xor_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::Bar) => {
                    self.consume();
                    let rhs_id = self.parse_xor_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Or,
                        rhs_id,
                    })
                }
                None => {
                    panic!("binary or expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_xor_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_and_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::Caret) => {
                    self.consume();
                    let rhs_id = self.parse_and_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Xor,
                        rhs_id,
                    })
                }
                None => {
                    panic!("xor expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_and_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_equality_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::Amp) => {
                    self.consume();
                    let rhs_id = self.parse_equality_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::And,
                        rhs_id,
                    })
                }
                None => {
                    panic!("binary and expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_equality_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_relational_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::EqualEqual) => {
                    self.consume();
                    let rhs_id = self.parse_relational_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Equal,
                        rhs_id,
                    })
                }
                Some(Token::ExclEqual) => {
                    self.consume();
                    let rhs_id = self.parse_relational_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::NotEqual,
                        rhs_id,
                    })
                }
                None => {
                    panic!("equality expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_relational_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_shift_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::Less) => {
                    self.consume();
                    let rhs_id = self.parse_shift_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::LessThan,
                        rhs_id,
                    })
                }
                Some(Token::Greater) => {
                    self.consume();
                    let rhs_id = self.parse_shift_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::GreaterThan,
                        rhs_id,
                    })
                }
                Some(Token::LessEqual) => {
                    self.consume();
                    let rhs_id = self.parse_shift_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::LessThanOrEqual,
                        rhs_id,
                    })
                }
                Some(Token::GreaterEqual) => {
                    self.consume();
                    let rhs_id = self.parse_shift_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::GreaterThanOrEqual,
                        rhs_id,
                    })
                }
                None => {
                    panic!("relational expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_shift_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_additive_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::LessLess) => {
                    self.consume();
                    let rhs_id = self.parse_additive_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::ShiftLeft,
                        rhs_id,
                    })
                }
                Some(Token::GreaterGreater) => {
                    self.consume();
                    let rhs_id = self.parse_additive_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::ShiftRight,
                        rhs_id,
                    })
                }
                None => {
                    panic!("shift expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_additive_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_multiplicative_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::Plus) => {
                    self.consume();
                    let rhs_id = self.parse_multiplicative_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Add,
                        rhs_id,
                    })
                }
                Some(Token::Minus) => {
                    self.consume();
                    let rhs_id = self.parse_multiplicative_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Subtract,
                        rhs_id,
                    })
                }
                None => {
                    panic!("additive expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_multiplicative_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        let expr_id = self.parse_unary_expression(ctrl_id);
        let mut total_id = self.exprs.peephole(expr_id);
        loop {
            match self.current {
                Some(Token::Star) => {
                    self.consume();
                    let rhs_id = self.parse_unary_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Multiply,
                        rhs_id,
                    })
                }
                Some(Token::Slash) => {
                    self.consume();
                    let rhs_id = self.parse_unary_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Divide,
                        rhs_id,
                    })
                }
                Some(Token::Percent) => {
                    self.consume();
                    let rhs_id = self.parse_unary_expression(ctrl_id);
                    let rhs_id = self.exprs.peephole(rhs_id);
                    total_id = self.exprs.alloc(Expr::Binary {
                        lhs_id: total_id,
                        op: BinaryOp::Modulo,
                        rhs_id,
                    })
                }
                None => {
                    panic!("multiplicative expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total_id
    }

    fn parse_unary_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        match self.current {
            Some(Token::Minus) => {
                self.consume();
                let expr_id = self.parse_unary_expression(ctrl_id);
                let expr_id = self.exprs.peephole(expr_id);
                self.exprs.alloc(Expr::Unary {
                    op: UnaryOp::Negate,
                    expr_id,
                })
            }
            Some(Token::Excl) => {
                self.consume();
                let expr_id = self.parse_unary_expression(ctrl_id);
                let expr_id = self.exprs.peephole(expr_id);
                self.exprs.alloc(Expr::Unary {
                    op: UnaryOp::Not,
                    expr_id,
                })
            }
            _ => self.parse_primary_expression(ctrl_id),
        }
    }

    fn parse_primary_expression(&mut self, ctrl_id: Id<Ctrl>) -> Id<Expr> {
        if let Some(Token::OpenParen) = self.current {
            self.consume();
            let expr_id = self.parse_expression(ctrl_id);
            let expr_id = self.exprs.peephole(expr_id);
            if let Some(Token::CloseParen) = self.current {
                self.consume();
                expr_id
            } else {
                panic!("primary expression: `)` expected");
            }
        } else {
            match &self.current {
                Some(Token::Integer(value)) => {
                    let value = *value;
                    self.consume();
                    self.exprs.alloc(Expr::Constant {
                        value: Value::Int(IntValue::Constant(value)),
                    })
                }
                Some(Token::Identifier(name)) => {
                    let name = name.clone();
                    self.consume();
                    if let Some(expr_id) = self.ctrls.symbols(ctrl_id).get(&name) {
                        expr_id
                    } else {
                        panic!("primary expression: undefined identifier `{}`", name);
                    }
                }
                Some(token) => {
                    panic!("primary expression: unexpected `{}`", token);
                }
                None => {
                    panic!("primary expression: unexpected end of source");
                }
            }
        }
    }
}
