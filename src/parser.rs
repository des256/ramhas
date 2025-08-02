use {
    crate::*,
    std::{cell::RefCell, path::Path, rc::Rc},
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(source);
        let current = tokenizer.next();
        Self { tokenizer, current }
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

    pub fn program(&mut self) -> Rc<RefCell<Ctrl>> {
        let ctrl = Rc::new(RefCell::new(Ctrl::Start {
            args: vec![],
            scopes: Scopes::new(),
        }));
        if let Ctrl::Start { ref mut scopes, .. } = &mut *ctrl.borrow_mut() {
            scopes.push();
        }
        // TODO: add args to scope as Proj
        let mut result: Option<Rc<RefCell<Ctrl>>> = None;
        let mut i = 0;
        while let Some(_) = self.current {
            result = self.statement(&ctrl);
            if let Some(result) = &result {
                visualize(&result, Path::new(&format!("test{}.svg", i))).unwrap();
            } else {
                visualize(&ctrl, Path::new(&format!("test{}.svg", i))).unwrap();
            }
            i += 1;
        }
        if let Ctrl::Start { ref mut scopes, .. } = &mut *ctrl.borrow_mut() {
            scopes.pop();
        }
        if let Some(result) = result {
            result
        } else {
            panic!("program: return statement expected");
        }
    }

    fn statement(&mut self, ctrl: &Rc<RefCell<Ctrl>>) -> Option<Rc<RefCell<Ctrl>>> {
        match &self.current {
            Some(Token::Return) => Some(self.return_statement(ctrl)),
            Some(Token::Int) => {
                self.declaration_statement(ctrl);
                None
            }
            Some(Token::OpenBrace) => {
                self.block_statement(ctrl);
                None
            }
            Some(Token::Identifier(name)) => {
                self.expression_statement(ctrl, &name.clone());
                None
            }
            Some(token) => panic!("statement: unexpected `{}`", token),
            None => panic!("statement: unexpected end of source"),
        }
    }

    fn return_statement(&mut self, ctrl: &Rc<RefCell<Ctrl>>) -> Rc<RefCell<Ctrl>> {
        self.expect(Token::Return);
        let expr = self.expression(ctrl);
        let result = Rc::new(RefCell::new(Ctrl::Return {
            ctrl: ctrl.clone(),
            expr,
        }));
        self.expect(Token::Semicolon);
        result
    }

    fn declaration_statement(&mut self, ctrl: &Rc<RefCell<Ctrl>>) {
        self.expect(Token::Int);
        let name = if let Some(Token::Identifier(name)) = &self.current {
            name.clone()
        } else {
            panic!("declaration statement: identifier expected");
        };
        self.consume(); // name
        self.expect(Token::Equal);
        let expr = self.expression(ctrl);
        self.expect(Token::Semicolon);
        ctrl.borrow_mut().declare_symbol(&name, expr);
    }

    fn block_statement(&mut self, ctrl: &Rc<RefCell<Ctrl>>) {
        self.expect(Token::OpenBrace);
        ctrl.borrow_mut().push_scope();
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
                    self.statement(ctrl);
                }
            }
        }
        ctrl.borrow_mut().pop_scope();
    }

    fn expression_statement(&mut self, ctrl: &Rc<RefCell<Ctrl>>, name: &str) {
        self.consume(); // identifier
        self.expect(Token::Equal);
        let expr = self.expression(ctrl);
        self.expect(Token::Semicolon);
        ctrl.borrow_mut().set_symbol(name, expr);
    }

    fn expression(&mut self, ctrl: &Rc<RefCell<Ctrl>>) -> Rc<RefCell<Expr>> {
        let expr = self.additive_expression(ctrl);
        //let new_expr = expr.borrow().peephole_optimize();
        //*expr.borrow_mut() = new_expr;
        expr
    }

    fn additive_expression(&mut self, ctrl: &Rc<RefCell<Ctrl>>) -> Rc<RefCell<Expr>> {
        let mut total = self.multiplicative_expression(ctrl);
        loop {
            match self.current {
                Some(Token::Plus) => {
                    self.consume();
                    let rhs = self.multiplicative_expression(ctrl);
                    total = Rc::new(RefCell::new(Expr::Add {
                        lhs: total,
                        rhs: rhs,
                    }))
                }
                Some(Token::Minus) => {
                    self.consume();
                    let rhs = self.multiplicative_expression(ctrl);
                    total = Rc::new(RefCell::new(Expr::Sub {
                        lhs: total,
                        rhs: rhs,
                    }))
                }
                None => {
                    panic!("additive expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total
    }

    fn multiplicative_expression(&mut self, ctrl: &Rc<RefCell<Ctrl>>) -> Rc<RefCell<Expr>> {
        let mut total = self.unary_expression(ctrl);
        loop {
            match self.current {
                Some(Token::Star) => {
                    self.consume();
                    let rhs = self.unary_expression(ctrl);
                    total = Rc::new(RefCell::new(Expr::Mul {
                        lhs: total,
                        rhs: rhs,
                    }))
                }
                Some(Token::Slash) => {
                    self.consume();
                    let rhs = self.unary_expression(ctrl);
                    total = Rc::new(RefCell::new(Expr::Div {
                        lhs: total,
                        rhs: rhs,
                    }))
                }
                None => {
                    panic!("multiplicative expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total
    }

    fn unary_expression(&mut self, ctrl: &Rc<RefCell<Ctrl>>) -> Rc<RefCell<Expr>> {
        if let Some(Token::Minus) = self.current {
            self.consume();
            let expr = self.unary_expression(ctrl);
            Rc::new(RefCell::new(Expr::Neg { expr }))
        } else {
            self.primary_expression(ctrl)
        }
    }

    fn primary_expression(&mut self, ctrl: &Rc<RefCell<Ctrl>>) -> Rc<RefCell<Expr>> {
        if let Some(Token::OpenParen) = self.current {
            self.consume();
            let expr = self.expression(ctrl);
            if let Some(Token::CloseParen) = self.current {
                self.consume();
                expr
            } else {
                panic!("primary expression: `)` expected");
            }
        } else {
            match &self.current {
                Some(Token::Integer(value)) => {
                    let value = *value;
                    self.consume();
                    Rc::new(RefCell::new(Expr::Constant {
                        value: Value {
                            ty: Ty::Int,
                            data: Data::Int(value),
                        },
                    }))
                }
                Some(Token::Identifier(name)) => {
                    let name = name.clone();
                    self.consume();
                    if let Some(expr) = ctrl.borrow().symbol(&name) {
                        expr
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
