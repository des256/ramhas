use {
    crate::*,
    std::{iter::Iterator, str::Chars},
};

pub struct Tokenizer<'a> {
    source: Chars<'a>,
    peeked: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut source = source.chars();
        let peeked = source.next();
        Self {
            source,
            peeked,
            line: 1,
            column: 1,
        }
    }

    fn consume(&mut self) {
        self.peeked = self.source.next();
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.peeked {
            match c {
                // whitespace
                ' ' | '\t' | '\n' => {
                    self.consume();
                    match c {
                        ' ' => self.column += 1,
                        '\t' => self.column += 8,
                        '\n' => {
                            self.column = 1;
                            self.line += 1;
                        }
                        _ => {}
                    }
                }

                // integer
                '0'..='9' => {
                    self.consume();
                    self.column += 1;
                    let mut number = c.to_digit(10).unwrap() as i64;
                    while let Some(c) = self.peeked {
                        if c.is_digit(10) {
                            self.consume();
                            self.column += 1;
                            number = number * 10 + c.to_digit(10).unwrap() as i64;
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Integer(number));
                }

                // identifier or keyword
                'A'..='Z' | 'a'..='z' => {
                    self.consume();
                    self.column += 1;
                    let mut identifier = c.to_string();
                    while let Some(c) = self.peeked {
                        if c.is_alphanumeric() {
                            self.consume();
                            self.column += 1;
                            identifier.push(c);
                        } else {
                            break;
                        }
                    }
                    match identifier.as_str() {
                        "fn" => return Some(Token::Fn),
                        "return" => return Some(Token::Return),
                        "int" => return Some(Token::Int),
                        "if" => return Some(Token::If),
                        "else" => return Some(Token::Else),
                        _ => return Some(Token::Identifier(identifier)),
                    }
                }

                // punctuation
                '(' | ')' | '{' | '}' | ';' | '+' | '-' | '*' | '/' | '=' | '!' | '<' | '>' => {
                    self.consume();
                    self.column += 1;
                    match c {
                        '(' => return Some(Token::OpenParen),
                        ')' => return Some(Token::CloseParen),
                        '{' => return Some(Token::OpenBrace),
                        '}' => return Some(Token::CloseBrace),
                        ';' => return Some(Token::Semicolon),
                        '*' => return Some(Token::Star),
                        '/' => return Some(Token::Slash),
                        '|' => match self.peeked {
                            Some('|') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::BarBar);
                            }
                            _ => return Some(Token::Bar),
                        },
                        '&' => match self.peeked {
                            Some('&') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::AmpAmp);
                            }
                            _ => return Some(Token::Amp),
                        },
                        '^' => return Some(Token::Caret),
                        '~' => return Some(Token::Tilde),
                        '%' => return Some(Token::Percent),
                        '+' => match self.peeked {
                            Some('+') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::PlusPlus);
                            }
                            _ => return Some(Token::Plus),
                        },
                        '-' => match self.peeked {
                            Some('>') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::MinusGreater);
                            }
                            Some('-') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::MinusMinus);
                            }
                            _ => return Some(Token::Minus),
                        },
                        '=' => match self.peeked {
                            Some('=') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::EqualEqual);
                            }
                            _ => return Some(Token::Equal),
                        },
                        '!' => match self.peeked {
                            Some('=') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::ExclEqual);
                            }
                            _ => return Some(Token::Excl),
                        },
                        '<' => match self.peeked {
                            Some('=') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::LessEqual);
                            }
                            Some('<') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::LessLess);
                            }
                            _ => return Some(Token::Less),
                        },
                        '>' => match self.peeked {
                            Some('=') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::GreaterEqual);
                            }
                            Some('>') => {
                                self.consume();
                                self.column += 1;
                                return Some(Token::GreaterGreater);
                            }
                            _ => return Some(Token::Greater),
                        },
                        _ => {}
                    }
                }
                _ => {
                    self.consume();
                    self.column += 1;
                    println!("{}:{}: unexpected character: {}", self.line, self.column, c);
                }
            }
        }
        return None;
    }
}
