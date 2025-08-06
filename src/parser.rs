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

    pub fn parse_program(&mut self) -> Rc<Ctrl> {
        let ctrl: Rc<Ctrl> = Rc::new(Ctrl::Start {
            args: Vec::new(),
            bindings: Rc::new(RefCell::new(Vec::new())),
        });

        ctrl.push_scope();

        // TODO: add args to scope as Proj
        let mut result: Option<Rc<Ctrl>> = None;
        while let Some(_) = self.current {
            result = self.parse_statement(&ctrl);
        }

        ctrl.pop_scope();

        if let Some(result) = result {
            result
        } else {
            panic!("program: return statement expected");
        }
    }

    fn parse_statement(&mut self, ctrl: &Rc<Ctrl>) -> Option<Rc<Ctrl>> {
        #[allow(unused_assignments)]
        let mut title = String::new();
        let result: Option<Rc<Ctrl>> = match &self.current {
            Some(Token::Return) => {
                self.consume();
                let expr = self.parse_expression(ctrl).peephole();
                title = format!("return {};", expr);
                let result = Ctrl::new_return(Rc::clone(&ctrl), expr);
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
                let expr = self.parse_expression(ctrl).peephole();
                title = format!("int {} = {};", name, expr);
                self.expect(Token::Semicolon);
                ctrl.declare(&name, expr);
                None
            }
            Some(Token::OpenBrace) => {
                self.consume();
                ctrl.push_scope();
                let mut result: Option<Rc<Ctrl>> = None;
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
                            result = self.parse_statement(&ctrl);
                        }
                    }
                }
                ctrl.pop_scope();
                title = "{}".to_string();
                result
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.consume(); // identifier
                self.expect(Token::Equal);
                let expr = self.parse_expression(ctrl).peephole();
                title = format!("{} = {};", name, expr);
                self.expect(Token::Semicolon);
                ctrl.set(&name, expr);
                None
            }
            Some(token) => panic!("statement: unexpected `{}`", token),
            None => panic!("statement: unexpected end of source"),
        };
        if let Some(result) = &result {
            let result = Rc::clone(&result);
            visualize(&result, &title, Path::new(&format!("test{}.svg", self.pi))).unwrap();
        } else {
            visualize(&ctrl, &title, Path::new(&format!("test{}.svg", self.pi))).unwrap();
        }
        self.pi += 1;
        if let Some(result) = result {
            Some(result)
        } else {
            None
        }
    }

    fn parse_expression(&mut self, ctrl: &Rc<Ctrl>) -> Rc<Expr> {
        let mut total = self.parse_multiplicative_expression(ctrl).peephole();
        loop {
            match self.current {
                Some(Token::Plus) => {
                    self.consume();
                    let rhs = self.parse_multiplicative_expression(ctrl).peephole();
                    total = Rc::new(Expr::Add { lhs: total, rhs })
                }
                Some(Token::Minus) => {
                    self.consume();
                    let rhs = self.parse_multiplicative_expression(ctrl).peephole();
                    total = Rc::new(Expr::Subtract { lhs: total, rhs })
                }
                None => {
                    panic!("additive expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total
    }

    fn parse_multiplicative_expression(&mut self, ctrl: &Rc<Ctrl>) -> Rc<Expr> {
        let mut total = self.parse_unary_expression(ctrl).peephole();
        loop {
            match self.current {
                Some(Token::Star) => {
                    self.consume();
                    let rhs = self.parse_unary_expression(ctrl).peephole();
                    total = Rc::new(Expr::Multiply { lhs: total, rhs })
                }
                Some(Token::Slash) => {
                    self.consume();
                    let rhs = self.parse_unary_expression(ctrl).peephole();
                    total = Rc::new(Expr::Divide { lhs: total, rhs })
                }
                None => {
                    panic!("multiplicative expression: unexpected end of source");
                }
                _ => break,
            }
        }
        total
    }

    fn parse_unary_expression(&mut self, ctrl: &Rc<Ctrl>) -> Rc<Expr> {
        if let Some(Token::Minus) = self.current {
            self.consume();
            let expr = self.parse_unary_expression(ctrl).peephole();
            Rc::new(Expr::Negate { expr })
        } else {
            self.parse_primary_expression(ctrl)
        }
    }

    fn parse_primary_expression(&mut self, ctrl: &Rc<Ctrl>) -> Rc<Expr> {
        if let Some(Token::OpenParen) = self.current {
            self.consume();
            let expr = self.parse_expression(ctrl).peephole();
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
                    Rc::new(Expr::Constant { value })
                }
                Some(Token::Identifier(name)) => {
                    let name = name.clone();
                    self.consume();
                    if let Some(expr) = ctrl.get(&name) {
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
