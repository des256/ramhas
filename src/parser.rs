use {
    crate::*,
    std::{cell::RefCell, rc::Rc},
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: Option<Token>,
    scopes: Scopes,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(source);
        let current = tokenizer.next();
        let scopes = Scopes::new();
        Self {
            tokenizer,
            current,
            scopes,
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

    pub fn program(&mut self) -> Rc<RefCell<Node>> {
        let ctrl = Rc::new(RefCell::new(Node::Start));
        self.scopes.push();
        let mut result: Option<Rc<RefCell<Node>>> = None;
        while let Some(_) = self.current {
            result = self.statement(&ctrl);
        }
        self.scopes.pop();
        if let Some(result) = result {
            result
        } else {
            panic!("program: return statement expected");
        }
    }

    fn statement(&mut self, ctrl: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
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

    fn return_statement(&mut self, ctrl: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        self.expect(Token::Return);
        let node = self.expression(ctrl);
        let node = Rc::new(RefCell::new(Node::Return {
            ctrl: ctrl.clone(),
            node,
        }));
        self.expect(Token::Semicolon);
        node
    }

    fn declaration_statement(&mut self, ctrl: &Rc<RefCell<Node>>) {
        self.expect(Token::Int);
        let name = if let Some(Token::Identifier(name)) = &self.current {
            name.clone()
        } else {
            panic!("declaration statement: identifier expected");
        };
        self.consume(); // name
        self.expect(Token::Equal);
        let node = self.expression(ctrl);
        self.expect(Token::Semicolon);
        self.scopes.declare(&name, node);
    }

    fn block_statement(&mut self, ctrl: &Rc<RefCell<Node>>) {
        self.expect(Token::OpenBrace);
        self.scopes.push();
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
        self.scopes.pop();
    }

    fn expression_statement(&mut self, ctrl: &Rc<RefCell<Node>>, name: &str) {
        self.consume(); // identifier
        self.expect(Token::Equal);
        let node = self.expression(ctrl);
        self.expect(Token::Semicolon);
        self.scopes.set(name, node);
    }

    fn expression(&mut self, ctrl: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let node = self.additive_expression(ctrl);
        node.borrow_mut().optimize();
        node
    }

    fn additive_expression(&mut self, ctrl: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut total = self.multiplicative_expression(ctrl);
        loop {
            match self.current {
                Some(Token::Plus) => {
                    self.consume();
                    let rhs = self.multiplicative_expression(ctrl);
                    total = Rc::new(RefCell::new(Node::Add {
                        ctrl: ctrl.clone(),
                        lhs: total,
                        rhs: rhs,
                    }))
                }
                Some(Token::Minus) => {
                    self.consume();
                    let rhs = self.multiplicative_expression(ctrl);
                    total = Rc::new(RefCell::new(Node::Sub {
                        ctrl: ctrl.clone(),
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

    fn multiplicative_expression(&mut self, ctrl: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut total = self.unary_expression(ctrl);
        loop {
            match self.current {
                Some(Token::Star) => {
                    self.consume();
                    let rhs = self.unary_expression(ctrl);
                    total = Rc::new(RefCell::new(Node::Mul {
                        ctrl: ctrl.clone(),
                        lhs: total,
                        rhs: rhs,
                    }))
                }
                Some(Token::Slash) => {
                    self.consume();
                    let rhs = self.unary_expression(ctrl);
                    total = Rc::new(RefCell::new(Node::Div {
                        ctrl: ctrl.clone(),
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

    fn unary_expression(&mut self, ctrl: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        if let Some(Token::Minus) = self.current {
            self.consume();
            let node = self.unary_expression(ctrl);
            Rc::new(RefCell::new(Node::Neg {
                ctrl: ctrl.clone(),
                node: node,
            }))
        } else {
            self.primary_expression(ctrl)
        }
    }

    fn primary_expression(&mut self, ctrl: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        if let Some(Token::OpenParen) = self.current {
            self.consume();
            let node = self.expression(ctrl);
            if let Some(Token::CloseParen) = self.current {
                self.consume();
                node
            } else {
                panic!("primary expression: `)` expected");
            }
        } else {
            match &self.current {
                Some(Token::Integer(value)) => {
                    let value = *value;
                    self.consume();
                    Rc::new(RefCell::new(Node::Constant { value }))
                }
                Some(Token::Identifier(name)) => {
                    let name = name.clone();
                    self.consume();
                    if let Some(node) = self.scopes.get(&name) {
                        node
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
