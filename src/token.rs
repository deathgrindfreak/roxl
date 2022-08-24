#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
    pub line: u32,
}
