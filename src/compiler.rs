use std::str;

#[derive(Default)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater,
    Less, GreaterEqual, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, For, Fun, If, Nil, Or, Print,
    Return, Super, This, True, Var, While,

    EOF,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
    pub line: u32,
}

pub enum ScanError {
    UnexpectedCharacter,
    ExpectedMoreInput,
}

impl <'a> Scanner<'a> {
    pub fn compile(source: &'a str) -> Result<(), ScanError> {
        let mut s = Scanner::new(source);

        let mut line: Option<u32> = None;
        loop {
            let token = s.scan_token()?;
            if Some(token.line) != line {
                println!("{:0<4}", token.line);
                line = Some(token.line);
            } else {
                println!("    | ");
            }

            if token.token_type == TokenType::EOF { break; }
        }

        Ok(())
    }

    pub fn new(source: &'a str) -> Self {
        Scanner {source, start: 0, current: 0, line: 1}
    }

    pub fn scan_token(&mut self) -> Result<Token<'a>, ScanError> {
        self.skip_whitespace()?;
        self.start = self.current;

        if self.is_at_end() { return Ok(self.make_token(TokenType::EOF)); }

        match self.advance()? {
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            ';' => Ok(self.make_token(TokenType::Semicolon)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),
            '-' => Ok(self.make_token(TokenType::Minus)),
            '+' => Ok(self.make_token(TokenType::Plus)),
            '/' => Ok(self.make_token(TokenType::Slash)),
            '*' => Ok(self.make_token(TokenType::Star)),
            '!' => {
                let token_type = if self.match_char('=')? { TokenType::BangEqual } else { TokenType::Bang };
                Ok(self.make_token(token_type))
            },
            '=' => {
                let token_type = if self.match_char('=')? { TokenType::EqualEqual } else { TokenType::Equal };
                Ok(self.make_token(token_type))
            },
            '<' => {
                let token_type = if self.match_char('=')? { TokenType::LessEqual } else { TokenType::Less };
                Ok(self.make_token(token_type))
            },
            '>' => {
                let token_type = if self.match_char('=')? { TokenType::GreaterEqual } else { TokenType::Equal };
                Ok(self.make_token(token_type))
            },
            _ => Err(ScanError::UnexpectedCharacter)
        }
    }

    fn skip_whitespace(&mut self) -> Result<(), ScanError> {
        loop {
            match self.peek()? {
                ' ' | '\r' | '\t' => { self.advance()?; },
                '\n' => {
                    self.line += 1;
                    self.advance()?;
                },
                '/' => {
                    if self.peek_next()? == Some('/') {
                        while self.peek()? != '\n' && !self.is_at_end() { self.advance()?; }
                    } else {
                        return Ok(());
                    }
                }
                _ => { return Ok(()); },
            }
        }
    }

    fn match_char(&mut self, expected: char) -> Result<bool, ScanError> {
        Ok(
            if self.is_at_end() || self.peek()? != expected {
                false
            } else {
                self.current += 1;
                true
            }
        )
    }

    fn advance(&mut self) -> Result<char, ScanError> {
        self.current += 1;
        self.peek_nth(-1)
    }

    fn peek_next(&self) -> Result<Option<char>, ScanError> {
        Ok(
            if self.is_at_end() {
                None
            } else {
                Some(self.peek_nth(1)?)
            }
        )
    }

    fn peek(&self) -> Result<char, ScanError> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, offset: i32) -> Result<char, ScanError> {
        self.source.chars()
                   .nth(
                       ((self.current as i32) + offset)
                           .try_into()
                           .map_err(|_| ScanError::ExpectedMoreInput)?
                   )
                   .ok_or(ScanError::ExpectedMoreInput)
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            literal: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }
}
