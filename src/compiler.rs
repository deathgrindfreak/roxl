use std::str;
use crate::token::{Token, TokenType};
use crate::scanner::{ScanError, Scanner};
use crate::chunk::{Chunk, OpCode, Value};
use crate::precedence::Precedence;

pub fn compile(source: &str, chunk: &mut Chunk) -> Result<(), ScanError> {
    let mut p = Parser::new(source, chunk);

    p.advance();
    p.expression();
    p.consume(TokenType::EOF, "Expect end of expression.");
    p.emit_byte(OpCode::Return);

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
struct Rule<'a, 'b>(Option<Box<ParserFn<'a, 'b>>>, Option<Box<ParserFn<'a, 'b>>>, Precedence);

fn get_rule<'a, 'b>(token_type: TokenType) -> Rule<'a, 'b> {
    match token_type {
        TokenType::LeftParen => Rule(Some(Box::new(Parser::<'a>::grouping)), None, Precedence::None),
        TokenType::RightParen => Rule(None, None, Precedence::None),
        TokenType::LeftBrace => Rule(None, None, Precedence::None),
        TokenType::RightBrace => Rule(None, None, Precedence::None),
        TokenType::Comma => Rule(None, None, Precedence::None),
        TokenType::Dot => Rule(None, None, Precedence::None),
        TokenType::Minus => Rule(Some(Box::new(Parser::<'a>::unary)), Some(Box::new(Parser::<'a>::binary)), Precedence::Term),
        TokenType::Plus => Rule(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Term),
        TokenType::Semicolon => Rule(None, None, Precedence::None),
        TokenType::Slash => Rule(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Factor),
        TokenType::Star => Rule(None, Some(Box::new(Parser::<'a>::binary)), Precedence::Factor),
        TokenType::Bang => Rule(None, None, Precedence::None),
        TokenType::BangEqual => Rule(None, None, Precedence::None),
        TokenType::Equal => Rule(None, None, Precedence::None),
        TokenType::EqualEqual => Rule(None, None, Precedence::None),
        TokenType::Greater => Rule(None, None, Precedence::None),
        TokenType::Less => Rule(None, None, Precedence::None),
        TokenType::GreaterEqual => Rule(None, None, Precedence::None),
        TokenType::LessEqual => Rule(None, None, Precedence::None),
        TokenType::Identifier => Rule(None, None, Precedence::None),
        TokenType::String => Rule(None, None, Precedence::None),
        TokenType::Number => Rule(Some(Box::new(Parser::<'a>::number)), None, Precedence::None),
        TokenType::And => Rule(None, None, Precedence::None),
        TokenType::Class => Rule(None, None, Precedence::None),
        TokenType::Else => Rule(None, None, Precedence::None),
        TokenType::False => Rule(None, None, Precedence::None),
        TokenType::For => Rule(None, None, Precedence::None),
        TokenType::Fun => Rule(None, None, Precedence::None),
        TokenType::If => Rule(None, None, Precedence::None),
        TokenType::Nil => Rule(None, None, Precedence::None),
        TokenType::Or => Rule(None, None, Precedence::None),
        TokenType::Print => Rule(None, None, Precedence::None),
        TokenType::Return => Rule(None, None, Precedence::None),
        TokenType::Super => Rule(None, None, Precedence::None),
        TokenType::This => Rule(None, None, Precedence::None),
        TokenType::True => Rule(None, None, Precedence::None),
        TokenType::Var => Rule(None, None, Precedence::None),
        TokenType::While => Rule(None, None, Precedence::None),
        TokenType::EOF => Rule(None, None, Precedence::None),
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
        let operator_type = self.get_previous().token_type;

        let Rule(_, _, precedence) = get_rule(operator_type);
        self.parse_precedence(precedence + 1);

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            _ => {}
        }
    }

    pub fn unary(&mut self) {
        let operator_type = self.get_previous().token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpCode::Negate);
            },
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        match get_rule(self.get_previous().token_type) {
            Rule(Some(prefix_rule), _, _) => prefix_rule(self),
            _ => self.error("Expect expression."),
        }
    }

    pub fn number(&mut self) {
        self.emit_constant(
            self.get_previous()
                .literal
                .parse()
                .expect("Expected number")
        )
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

    pub fn get_previous(&self) -> &Token {
        self.previous.as_ref().expect("Expected previous token")
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
