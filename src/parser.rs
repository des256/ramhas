use {crate::*, generational_arena::Index as GenIndex};

pub struct Parser<'a> {
    arena: Arena,
    tokenizer: Tokenizer<'a>,
    peeked: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let arena = Arena::new();
        let mut tokenizer = Tokenizer::new(source);
        let peeked = tokenizer.next();
        Self {
            arena,
            tokenizer,
            peeked,
        }
    }

    pub fn print_state(&self) {
        println!("arena:");
        self.arena.print_state();
    }

    fn consume(&mut self) {
        self.peeked = self.tokenizer.next();
    }

    pub fn parse_program(&mut self) -> GenIndex {
        let ctrl = self.arena.insert(Node::Start);
        if let Some(Token::Return) = self.peeked {
            self.parse_return_statement(ctrl)
        } else {
            panic!("`return` expected");
        }
    }

    fn parse_return_statement(&mut self, ctrl: GenIndex) -> GenIndex {
        self.consume();
        let id = self.parse_expression();
        let id = self.arena.insert(Node::Return { ctrl, id });
        if let Some(Token::Semicolon) = self.peeked {
            self.consume();
        } else {
            panic!("`;` expected");
        }
        id
    }

    fn parse_expression(&mut self) -> GenIndex {
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> GenIndex {
        let mut id = self.parse_multiplicative_expression();
        loop {
            match self.peeked {
                Some(Token::Plus) => {
                    self.consume();
                    let id2 = self.parse_multiplicative_expression();
                    if let (Some(ref mut node), Some(ref mut node2)) = self.arena.get2_mut(id, id2)
                    {
                        if let Node::Constant { ref mut value } = node {
                            if let Node::Constant {
                                value: ref mut value2,
                            } = node2
                            {
                                *value += *value2;
                                self.arena.remove(id2);
                            } else {
                                id = self.arena.insert(Node::Add { id, id2 });
                            }
                        } else {
                            id = self.arena.insert(Node::Add { id, id2 });
                        }
                    } else {
                        panic!("can't find nodes {:?} and {:?}", id, id2);
                    }
                }
                Some(Token::Minus) => {
                    self.consume();
                    let id2 = self.parse_multiplicative_expression();
                    if let (Some(ref mut node), Some(ref mut node2)) = self.arena.get2_mut(id, id2)
                    {
                        if let Node::Constant { ref mut value } = node {
                            if let Node::Constant {
                                value: ref mut value2,
                            } = node2
                            {
                                *value -= *value2;
                                self.arena.remove(id2);
                            } else {
                                id = self.arena.insert(Node::Sub { id, id2 });
                            }
                        } else {
                            id = self.arena.insert(Node::Sub { id, id2 });
                        }
                    } else {
                        panic!("can't find nodes {:?} and {:?}", id, id2);
                    }
                }
                None => {
                    panic!("unexpected end of source");
                }
                _ => break,
            }
        }
        id
    }

    fn parse_multiplicative_expression(&mut self) -> GenIndex {
        let mut id = self.parse_unary_expression();
        loop {
            match self.peeked {
                Some(Token::Star) => {
                    self.consume();
                    let id2 = self.parse_unary_expression();
                    if let (Some(ref mut node), Some(ref mut node2)) = self.arena.get2_mut(id, id2)
                    {
                        if let Node::Constant { ref mut value } = node {
                            if let Node::Constant {
                                value: ref mut value2,
                            } = node2
                            {
                                *value *= *value2;
                                self.arena.remove(id2);
                            } else {
                                id = self.arena.insert(Node::Mul { id2, id });
                            }
                        } else {
                            id = self.arena.insert(Node::Mul { id, id2 });
                        }
                    } else {
                        panic!("can't find nodes {:?} and {:?}", id, id2);
                    }
                }
                Some(Token::Slash) => {
                    self.consume();
                    let id2 = self.parse_unary_expression();
                    if let (Some(ref mut node), Some(ref mut node2)) = self.arena.get2_mut(id, id2)
                    {
                        if let Node::Constant { ref mut value } = node {
                            if let Node::Constant {
                                value: ref mut value2,
                            } = node2
                            {
                                *value /= *value2;
                                self.arena.remove(id2);
                            } else {
                                id = self.arena.insert(Node::Div { id, id2 });
                            }
                        } else {
                            id = self.arena.insert(Node::Div { id, id2 });
                        }
                    } else {
                        panic!("can't find nodes {:?} and {:?}", id, id2);
                    }
                }
                None => {
                    panic!("unexpected end of source");
                }
                _ => break,
            }
        }
        id
    }

    fn parse_unary_expression(&mut self) -> GenIndex {
        if let Some(Token::Minus) = self.peeked {
            self.consume();
            let id = self.parse_unary_expression();
            if let Node::Constant { ref mut value } = self.arena.get_mut(id) {
                *value = -*value;
                id
            } else {
                self.arena.insert(Node::Neg { id })
            }
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_primary_expression(&mut self) -> GenIndex {
        if let Some(Token::OpenParen) = self.peeked {
            self.consume();
            let id = self.parse_expression();
            if let Some(Token::CloseParen) = self.peeked {
                self.consume();
                return id;
            } else {
                panic!("')' expected");
            }
        } else {
            match &self.peeked {
                Some(Token::Integer(value)) => {
                    let value = *value;
                    self.consume();
                    self.arena.insert(Node::Constant { value })
                }
                Some(token) => {
                    panic!("unexpected `{}`", token);
                }
                None => {
                    panic!("unexpected end of source");
                }
            }
        }
    }
}
