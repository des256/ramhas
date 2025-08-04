use {
    crate::*,
    std::{cell::RefCell, path::Path, rc::Rc},
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: Option<Token>,
    pi: usize,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(source);
        let current = tokenizer.next();
        Self {
            tokenizer,
            current,
            pi: 0,
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

    pub fn program(&mut self) -> Rc<RefCell<dyn Ctrl>> {
        let ctrl: Rc<RefCell<dyn Ctrl>> = Rc::new(RefCell::new(ctrl::Start::new()));
        {
            let mut ctrl_borrowed = ctrl.borrow_mut();
            (&mut *ctrl_borrowed)
                .as_any()
                .downcast_mut::<ctrl::Start>()
                .unwrap()
                .push_scope();
        }

        // TODO: add args to scope as Proj
        let mut result: Option<Rc<RefCell<dyn Ctrl>>> = None;
        while let Some(_) = self.current {
            result = self.statement(&ctrl);
        }
        {
            let mut ctrl_borrowed = ctrl.borrow_mut();
            (&mut *ctrl_borrowed)
                .as_any()
                .downcast_mut::<ctrl::Start>()
                .unwrap()
                .pop_scope();
        }

        if let Some(result) = result {
            result
        } else {
            panic!("program: return statement expected");
        }
    }

    fn statement(&mut self, ctrl: &Rc<RefCell<dyn Ctrl>>) -> Option<Rc<RefCell<dyn Ctrl>>> {
        let mut title = String::new();
        let result = match &self.current {
            Some(Token::Return) => {
                self.consume();
                let expr = self.expression(ctrl);
                title = format!("return {};", expr.borrow().to_string());
                let result = Rc::new(RefCell::new(ctrl::Return::new(ctrl.clone(), expr)));
                self.expect(Token::Semicolon);
                Some(result)
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
                let expr = self.expression(ctrl);
                title = format!("int {} = {};", name, expr.borrow().to_string());
                self.expect(Token::Semicolon);
                {
                    let mut ctrl_borrowed = ctrl.borrow_mut();
                    (&mut *ctrl_borrowed)
                        .as_any()
                        .downcast_mut::<ctrl::Start>()
                        .unwrap()
                        .declare(&name, expr);
                }
                None
            }
            Some(Token::OpenBrace) => {
                self.consume();
                {
                    let mut ctrl_borrowed = ctrl.borrow_mut();
                    (&mut *ctrl_borrowed)
                        .as_any()
                        .downcast_mut::<ctrl::Start>()
                        .unwrap()
                        .push_scope();
                }
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
                            let result = self.statement(&ctrl);
                        }
                    }
                }
                {
                    let mut ctrl_borrowed = ctrl.borrow_mut();
                    (&mut *ctrl_borrowed)
                        .as_any()
                        .downcast_mut::<ctrl::Start>()
                        .unwrap()
                        .pop_scope();
                }
                title = "{}".to_string();
                None
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.consume(); // identifier
                self.expect(Token::Equal);
                let expr = self.expression(ctrl);
                title = format!("{} = {};", name, expr.borrow().to_string());
                self.expect(Token::Semicolon);
                {
                    let mut ctrl_borrowed = ctrl.borrow_mut();
                    (&mut *ctrl_borrowed)
                        .as_any()
                        .downcast_mut::<ctrl::Start>()
                        .unwrap()
                        .set(&name, expr);
                }
                None
            }
            Some(token) => panic!("statement: unexpected `{}`", token),
            None => panic!("statement: unexpected end of source"),
        };
        if let Some(result) = &result {
            visualize(&result, &title, Path::new(&format!("test{}.svg", self.pi))).unwrap();
        } else {
            visualize(&ctrl, &title, Path::new(&format!("test{}.svg", self.pi))).unwrap();
        }
        self.pi += 1;
        if let Some(result) = result {
            Some(result as Rc<RefCell<dyn Ctrl>>)
        } else {
            None
        }
    }

    fn expression(&mut self, ctrl: &Rc<RefCell<dyn Ctrl>>) -> Rc<RefCell<dyn Expr>> {
        let expr = self.additive_expression(ctrl);
        //let new_expr = expr.borrow().peephole_optimize();
        //*expr.borrow_mut() = new_expr;
        expr
    }

    fn additive_expression(&mut self, ctrl: &Rc<RefCell<dyn Ctrl>>) -> Rc<RefCell<dyn Expr>> {
        let mut total = self.multiplicative_expression(ctrl);
        loop {
            match self.current {
                Some(Token::Plus) => {
                    self.consume();
                    let rhs = self.multiplicative_expression(ctrl);
                    total = Rc::new(RefCell::new(expr::Add::new(total, rhs)))
                }
                Some(Token::Minus) => {
                    self.consume();
                    let rhs = self.multiplicative_expression(ctrl);
                    total = Rc::new(RefCell::new(expr::Subtract::new(total, rhs)))
                }
                None => {
                    panic!("additive expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total
    }

    fn multiplicative_expression(&mut self, ctrl: &Rc<RefCell<dyn Ctrl>>) -> Rc<RefCell<dyn Expr>> {
        let mut total = self.unary_expression(ctrl);
        loop {
            match self.current {
                Some(Token::Star) => {
                    self.consume();
                    let rhs = self.unary_expression(ctrl);
                    total = Rc::new(RefCell::new(expr::Multiply::new(total, rhs)))
                }
                Some(Token::Slash) => {
                    self.consume();
                    let rhs = self.unary_expression(ctrl);
                    total = Rc::new(RefCell::new(expr::Divide::new(total, rhs)))
                }
                None => {
                    panic!("multiplicative expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total
    }

    fn unary_expression(&mut self, ctrl: &Rc<RefCell<dyn Ctrl>>) -> Rc<RefCell<dyn Expr>> {
        if let Some(Token::Minus) = self.current {
            self.consume();
            let expr = self.unary_expression(ctrl);
            Rc::new(RefCell::new(expr::Negate::new(expr)))
        } else {
            self.primary_expression(ctrl)
        }
    }

    fn primary_expression(&mut self, ctrl: &Rc<RefCell<dyn Ctrl>>) -> Rc<RefCell<dyn Expr>> {
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
                    Rc::new(RefCell::new(expr::Constant::new(value)))
                }
                Some(Token::Identifier(name)) => {
                    let name = name.clone();
                    self.consume();
                    if let Some(expr) = {
                        let mut ctrl_borrowed = ctrl.borrow_mut();
                        (&mut *ctrl_borrowed)
                            .as_any()
                            .downcast_ref::<ctrl::Start>()
                            .unwrap()
                            .get(&name)
                    } {
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
