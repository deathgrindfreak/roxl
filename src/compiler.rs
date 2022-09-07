use crate::value::{Value, ObjectType};
use crate::token::{Token, TokenType};
use crate::scanner::{ScanError, Scanner};
use crate::chunk::{Chunk, OpCode};
use crate::precedence::Precedence;

use std::str;

pub fn compile(source: &str, chunk: &mut Chunk) -> Result<(), ScanError> {
    let mut p = Parser::new(source, chunk);

    p.advance();
    p.expression();
    p.consume(TokenType::EOF, "Expect end of expression.");
    p.emit_return();

    Ok(())
}

#[derive(Debug)]
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    chunk: &'a mut Chunk,

    previous: Option<Token<'a>>,
    current: Option<Token<'a>>,

    had_error: bool,
    panic_mode: bool,
}

#[derive(Debug)]
pub enum ParseError {
    ScanError(ScanError)
}

impl From<ScanError> for ParseError {
    fn from(value: ScanError) -> ParseError {
        ParseError::ScanError(value)
    }
}

type ParserFn<'a, 'b> = fn(&'b mut Parser<'a>);
struct Rule<'a, 'b> {
    prefix: Option<Box<ParserFn<'a, 'b>>>,
    infix: Option<Box<ParserFn<'a, 'b>>>,
    precedence: Precedence
}

impl<'a, 'b> Rule<'a, 'b> {
    fn new(
        prefix: Option<Box<ParserFn<'a, 'b>>>,
        infix: Option<Box<ParserFn<'a, 'b>>>,
        precedence: Precedence
    ) -> Self {
        Rule { prefix, infix, precedence }
    }
}

fn get_rule<'a, 'b>(token_type: TokenType) -> Rule<'a, 'b> {
    match token_type {
        TokenType::LeftParen => Rule::new(Some(Box::new(Parser::<'a>::grouping)), None, Precedence::None),
        TokenType::RightParen => Rule::new(None, None, Precedence::None),
        TokenType::LeftBrace => Rule::new(None, None, Precedence::None),
        TokenType::RightBrace => Rule::new(None, None, Precedence::None),
        TokenType::Comma => Rule::new(None, None, Precedence::None),
        TokenType::Dot => Rule::new(None, None, Precedence::None),
        TokenType::Minus => Rule::new(Some(Box::new(Parser::<'a>::unary)), Some(Box::new(Parser::<'a>::binary)), Precedence::Term),
        TokenType::Plus => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Term),
        TokenType::Semicolon => Rule::new(None, None, Precedence::None),
        TokenType::Slash => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Factor),
        TokenType::Star => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Factor),
        TokenType::Bang => Rule::new(Some(Box::new(Parser::<'a>::unary)), None, Precedence::None),
        TokenType::BangEqual => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Equality),
        TokenType::Equal => Rule::new(None, None, Precedence::None),
        TokenType::EqualEqual => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Equality),
        TokenType::Greater => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Comparison),
        TokenType::Less => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Comparison),
        TokenType::GreaterEqual => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Comparison),
        TokenType::LessEqual => Rule::new(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Comparison),
        TokenType::Identifier => Rule::new(None, None, Precedence::None),
        TokenType::String => Rule::new(Some(Box::new(Parser::<'a>::string)), None, Precedence::None),
        TokenType::Number => Rule::new(Some(Box::new(Parser::<'a>::number)), None, Precedence::None),
        TokenType::And => Rule::new(None, None, Precedence::None),
        TokenType::Class => Rule::new(None, None, Precedence::None),
        TokenType::Else => Rule::new(None, None, Precedence::None),
        TokenType::False => Rule::new(Some(Box::new(Parser::<'a>::literal)), None, Precedence::None),
        TokenType::For => Rule::new(None, None, Precedence::None),
        TokenType::Fun => Rule::new(None, None, Precedence::None),
        TokenType::If => Rule::new(None, None, Precedence::None),
        TokenType::Nil => Rule::new(Some(Box::new(Parser::<'a>::literal)), None, Precedence::None),
        TokenType::Or => Rule::new(None, None, Precedence::None),
        TokenType::Print => Rule::new(None, None, Precedence::None),
        TokenType::Return => Rule::new(None, None, Precedence::None),
        TokenType::Super => Rule::new(None, None, Precedence::None),
        TokenType::This => Rule::new(None, None, Precedence::None),
        TokenType::True => Rule::new(Some(Box::new(Parser::<'a>::literal)), None, Precedence::None),
        TokenType::Var => Rule::new(None, None, Precedence::None),
        TokenType::While => Rule::new(None, None, Precedence::None),
        TokenType::EOF => Rule::new(None, None, Precedence::None),
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, chunk: &'a mut Chunk) -> Self {
        Parser {
            scanner: Scanner::new(source),
            chunk,
            previous: None,
            current: None,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    pub fn binary(&mut self) {
        let operator_type = self.previous().token_type;

        let Rule { precedence, .. } = get_rule(operator_type);
        self.parse_precedence(precedence + 1);

        match operator_type {
            TokenType::BangEqual => self.emit_bytes(OpCode::Equal, OpCode::Not),
            TokenType::EqualEqual => self.emit_byte(OpCode::Equal),
            TokenType::Greater => self.emit_byte(OpCode::Greater),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::Less, OpCode::Not),
            TokenType::Less => self.emit_byte(OpCode::Less),
            TokenType::LessEqual => self.emit_bytes(OpCode::Greater, OpCode::Not),
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            _ => {}
        }
    }

    pub fn unary(&mut self) {
        let operator_type = self.previous().token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::Not),
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        match get_rule(self.previous().token_type) {
            Rule { prefix: Some(prefix_rule), .. } => {
                prefix_rule(self);

                while precedence <= get_rule(self.get_current().token_type).precedence {
                    self.advance();
                    if let Rule { infix: Some(infix_rule), .. } = get_rule(self.previous().token_type) {
                        infix_rule(self);
                    }
                }
            },
            _ => self.error("Expect expression."),
        }
    }

    pub fn string(&mut self) {
        let p = self.previous().literal;
        self.emit_constant(
            Value::Object(
                // Truncate the quotation marks
                ObjectType::Str(p[1..p.len()-1].to_string())
            )
        );
    }

    pub fn number(&mut self) {
        self.emit_constant(
            self.previous()
                .literal
                .parse()
                .expect("Expected number")
        )
    }

    pub fn literal(&mut self) {
        match self.previous().token_type {
            TokenType::Nil => self.emit_byte(OpCode::Nil),
            TokenType::True => self.emit_byte(OpCode::True),
            TokenType::False => self.emit_byte(OpCode::False),
            _ => {},
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant.into(), constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        match self.chunk.add_constant(value).try_into() {
            Ok(c) => c,
            Err(_) => {
                self.error("Too many constants in one chunk.");
                0
            }
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_byte<U: Into<u8>>(&mut self, byte: U) {
        let line = self.previous.as_ref().unwrap().line;
        self.chunk.write(byte, line);
    }

    fn emit_bytes<U: Into<u8>>(&mut self, byte1: U, byte2: U) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn consume(&mut self, token_type: TokenType, message: &'a str) {
        if self.current.as_ref().map_or(false, |t| t.token_type == token_type) {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    pub fn previous(&self) -> &Token {
        self.previous.as_ref().expect("Expected previous token")
    }

    pub fn get_current(&self) -> &Token {
        self.current.as_ref().expect("Expected previous token")
    }

    pub fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            match self.scanner.scan_token() {
                Ok(token) =>  {
                    self.current = Some(token);
                    break;
                },
                Err(_e) => {
                    let message = self.current.as_ref().map_or("", |t| t.literal);
                    self.error_at_current(message);
                }
            }
        }
    }

    fn error_at_current(&mut self, message: &'a str) {
        // TODO Need to handle 'None'
        self.error_at(&self.current.clone().unwrap(), message)
    }

    fn error(&mut self, message: &'a str) {
        // TODO Need to handle 'None'
        self.error_at(&self.previous.clone().unwrap(), message)
    }

    fn error_at(&mut self, token: &Token, message: &'a str) {
        if self.panic_mode { return; }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::EOF {
            eprint!(" at end");
        } else {
            eprint!(" at '{}'", token.literal);
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        assert_expr("1 + 1", vec![
            OpCode::Constant.into(), 0x00,
            OpCode::Constant.into(), 0x01,
            OpCode::Add.into()
        ]);

        assert_expr("2 * 2", vec![
            OpCode::Constant.into(), 0x00,
            OpCode::Constant.into(), 0x01,
            OpCode::Multiply.into()
        ]);

        assert_expr("3 / 3", vec![
            OpCode::Constant.into(), 0x00,
            OpCode::Constant.into(), 0x01,
            OpCode::Divide.into()
        ]);

        assert_expr("4 - 4", vec![
            OpCode::Constant.into(), 0x00,
            OpCode::Constant.into(), 0x01,
            OpCode::Subtract.into()
        ]);
    }

    #[test]
    fn test_grouping() {
        assert_expr("(1 + 1) * 2", vec![
            OpCode::Constant.into(), 0x00,
            OpCode::Constant.into(), 0x01,
            OpCode::Add.into(),
            OpCode::Constant.into(), 0x02,
            OpCode::Multiply.into(),
        ]);

        assert_expr("(1 + 1) * (2 - 1) / 4", vec![
            OpCode::Constant.into(), 0x00,
            OpCode::Constant.into(), 0x01,
            OpCode::Add.into(),
            OpCode::Constant.into(), 0x02,
            OpCode::Constant.into(), 0x03,
            OpCode::Subtract.into(),
            OpCode::Multiply.into(),
            OpCode::Constant.into(), 0x04,
            OpCode::Divide.into(),
        ]);
    }

    fn assert_expr(source: &str, code: Vec<u8>) {
        let mut chunk = Chunk::default();
        let mut p = Parser::new(source, &mut chunk);

        p.advance();
        p.expression();
        p.consume(TokenType::EOF, "Expect end of expression.");

        eprintln!("{:?}", chunk);
        assert_eq!(chunk.code, code);
    }
}
