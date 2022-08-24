use crate::token::{Token, TokenType};

#[derive(Debug, Default)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: u32,
}

#[derive(Debug)]
pub enum ScanError {
    UnexpectedCharacter,
    ExpectedMoreInput,
    UnterminatedString,
    BadPeekOffset,
}

impl <'a> Scanner<'a> {
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
                let token_type = if self.match_char('=')? { TokenType::GreaterEqual } else { TokenType::Greater };
                Ok(self.make_token(token_type))
            },
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphabetic() => self.identifier(),
            _ => Err(ScanError::UnexpectedCharacter)
        }
    }

    fn identifier(&mut self) -> Result<Token<'a>, ScanError> {
        while self.check(|c| c.is_ascii_digit() || c.is_alphabetic())? {
            self.advance()?;
        }
        Ok(self.make_token(self.identifier_type()?))
    }

    fn identifier_type(&self) -> Result<TokenType, ScanError> {
        match self.source.chars().nth(self.start).ok_or(ScanError::BadPeekOffset)? {
            'a' => Ok(self.check_keyword(1, "nd", TokenType::And)),
            'c' => Ok(self.check_keyword(1, "lass", TokenType::Class)),
            'e' => Ok(self.check_keyword(1, "lse", TokenType::Else)),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).ok_or(ScanError::BadPeekOffset)? {
                        'a' => Ok(self.check_keyword(2, "lse", TokenType::False)),
                        'o' => Ok(self.check_keyword(2, "r", TokenType::For)),
                        'u' => Ok(self.check_keyword(2, "n", TokenType::Fun)),
                        _ => Ok(TokenType::Identifier),
                    }
                } else {
                    Ok(TokenType::Identifier)
                }
            }
            'i' => Ok(self.check_keyword(1, "f", TokenType::If)),
            'n' => Ok(self.check_keyword(1, "il", TokenType::Nil)),
            'o' => Ok(self.check_keyword(1, "r", TokenType::Or)),
            'p' => Ok(self.check_keyword(1, "rint", TokenType::Print)),
            'r' => Ok(self.check_keyword(1, "eturn", TokenType::Return)),
            's' => Ok(self.check_keyword(1, "uper", TokenType::Super)),
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).ok_or(ScanError::BadPeekOffset)? {
                        'h' => Ok(self.check_keyword(2, "is", TokenType::This)),
                        'r' => Ok(self.check_keyword(2, "ue", TokenType::True)),
                        _ => Ok(TokenType::Identifier),
                    }
                } else {
                    Ok(TokenType::Identifier)
                }
            }
            'v' => Ok(self.check_keyword(1, "ar", TokenType::Var)),
            'w' => Ok(self.check_keyword(1, "hile", TokenType::While)),
            _ => Ok(TokenType::Identifier),
        }
    }

    fn check_keyword(&self, start: usize, rest: &'a str, token_type: TokenType) -> TokenType {
        let offset = self.start + start;
        if self.current - self.start == start + rest.len() && self.source[offset..offset + rest.len()] == rest[..] {
            token_type
        } else {
            TokenType::Identifier
        }
    }

    fn string(&mut self) -> Result<Token<'a>, ScanError> {
        while self.check(|c| c != '"')? && !self.is_at_end() {
            if self.check(|c| c == '\n')? { self.line += 1; }
            self.advance()?;
        }

        if self.is_at_end() { return Err(ScanError::UnterminatedString) }
        self.advance()?;

        Ok(self.make_token(TokenType::String))
    }

    fn number(&mut self) -> Result<Token<'a>, ScanError> {
        while self.check(|c| c.is_ascii_digit())? { self.advance()?; }

        if self.check(|c| c == '.')? && self.check_next(|c| c.is_ascii_digit())? {
            self.advance()?;

            while self.check(|c| c.is_ascii_digit())? { self.advance()?; }
        }

        Ok(self.make_token(TokenType::Number))
    }

    fn skip_whitespace(&mut self) -> Result<(), ScanError> {
        loop {
            match self.peek()? {
                Some(' ') | Some('\r') | Some('\t') => { self.advance()?; },
                Some('\n') => {
                    self.line += 1;
                    self.advance()?;
                },
                Some('/') => {
                    if self.peek_next()? == Some('/') {
                        while self.check(|c| c != '\n')? && !self.is_at_end() { self.advance()?; }
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
            if self.is_at_end() || self.check(|c| c != expected)? {
                false
            } else {
                self.current += 1;
                true
            }
        )
    }

    fn advance(&mut self) -> Result<char, ScanError> {
        self.current += 1;
        Ok(self.peek_nth(-1)?.unwrap())
    }

    fn peek_next(&self) -> Result<Option<char>, ScanError> {
        Ok(
            if self.is_at_end() {
                None
            } else {
                self.peek_nth(1)?
            }
        )
    }

    fn check_next<F: Fn(char) -> bool>(&self, pred: F) -> Result<bool, ScanError> {
        Ok(self.peek_next()?.map(pred).unwrap_or(false))
    }

    fn check<F: Fn(char) -> bool>(&self, pred: F) -> Result<bool, ScanError> {
        Ok(self.peek()?.map(pred).unwrap_or(false))
    }

    fn peek(&self) -> Result<Option<char>, ScanError> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, offset: i32) -> Result<Option<char>, ScanError> {
        Ok(self.source.chars()
           .nth(((self.current as i32) + offset)
                .try_into()
                .map_err(|_| ScanError::BadPeekOffset)?))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primitives() {
        assert_eq!(test_scan_token("("), TokenType::LeftParen);
        assert_eq!(test_scan_token(")"), TokenType::RightParen);
        assert_eq!(test_scan_token("{"), TokenType::LeftBrace);
        assert_eq!(test_scan_token("}"), TokenType::RightBrace);
        assert_eq!(test_scan_token(";"), TokenType::Semicolon);
        assert_eq!(test_scan_token(","), TokenType::Comma);
        assert_eq!(test_scan_token("."), TokenType::Dot);
        assert_eq!(test_scan_token("-"), TokenType::Minus);
        assert_eq!(test_scan_token("+"), TokenType::Plus);
        assert_eq!(test_scan_token("/"), TokenType::Slash);
        assert_eq!(test_scan_token("*"), TokenType::Star);
        assert_eq!(test_scan_token("!"), TokenType::Bang);
        assert_eq!(test_scan_token("!="), TokenType::BangEqual);
        assert_eq!(test_scan_token("="), TokenType::Equal);
        assert_eq!(test_scan_token("=="), TokenType::EqualEqual);
        assert_eq!(test_scan_token("<"), TokenType::Less);
        assert_eq!(test_scan_token("<="), TokenType::LessEqual);
        assert_eq!(test_scan_token(">"), TokenType::Greater);
        assert_eq!(test_scan_token(">="), TokenType::GreaterEqual);
    }

    #[test]
    fn test_number() {
        test_scan("   123 ", "123", TokenType::Number);
        test_scan("   123.123 ", "123.123", TokenType::Number);
    }

    #[test]
    fn test_string() {
        test_scan("   \"blah\" ", "\"blah\"", TokenType::String);
        test_scan("
\"Here's a multiline
string\"
", "\"Here's a multiline\nstring\"", TokenType::String);
    }

    #[test]
    fn test_keywords() {
        test_scan("and", "and", TokenType::And);
        test_scan("class", "class", TokenType::Class);
        test_scan("else", "else", TokenType::Else);
        test_scan("false", "false", TokenType::False);
        test_scan("for", "for", TokenType::For);
        test_scan("fun", "fun", TokenType::Fun);
        test_scan("if", "if", TokenType::If);
        test_scan("nil", "nil", TokenType::Nil);
        test_scan("or", "or", TokenType::Or);
        test_scan("print", "print", TokenType::Print);
        test_scan("return", "return", TokenType::Return);
        test_scan("super", "super", TokenType::Super);
        test_scan("this", "this", TokenType::This);
        test_scan("true", "true", TokenType::True);
        test_scan("var", "var", TokenType::Var);
        test_scan("while", "while", TokenType::While);
    }

    #[test]
    fn test_identifier() {
        test_scan("   blah ", "blah", TokenType::Identifier);
        test_scan("   foo9000 ", "foo9000", TokenType::Identifier);
    }

    fn test_scan(input: &str, expected: &str, expected_type: TokenType) {
        eprintln!("{}", input);
        let Token {literal, token_type, ..} = Scanner::new(input).scan_token().unwrap();
        assert_eq!(literal, expected);
        assert_eq!(token_type, expected_type);
    }

    fn test_scan_token(input: &str) -> TokenType {
        Scanner::new(input).scan_token().unwrap().token_type
    }
}
