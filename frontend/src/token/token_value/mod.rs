mod literal;
mod r#type;

use std::fmt::{self, Display};

pub use self::{literal::Literal, r#type::Type};

#[derive(PartialEq, Debug)]
pub enum TokenValue {
    // keywords
    As,
    Break,
    Continue,
    Else,
    Fn,
    If,
    While,
    Let,
    Mut,
    Return,

    // special chars
    LParen,
    RParen,
    LBrace,
    RBrace,

    Semicolon,
    Colon,
    Arrow,
    Comma,

    Assignment,
    PlusAssignment,
    MinusAssignment,
    StarAssignment,
    SlashAssignment,

    Or,
    And,
    Not,

    NotEqual,
    Equal,

    GreaterOrEqual,
    Greater,
    LessOrEqual,
    Less,

    Plus,
    Minus,
    Star,
    Slash,

    // other
    Type(Type),
    Id(String),
    Literal(Literal),

    EOF,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValue::As => write!(f, "as"),
            TokenValue::Break => write!(f, "break"),
            TokenValue::Continue => write!(f, "continue"),
            TokenValue::Else => write!(f, "else"),
            TokenValue::Fn => write!(f, "fn"),
            TokenValue::If => write!(f, "if"),
            TokenValue::While => write!(f, "while"),
            TokenValue::Let => write!(f, "let"),
            TokenValue::Mut => write!(f, "mut"),
            TokenValue::Return => write!(f, "return"),
            TokenValue::LParen => write!(f, "("),
            TokenValue::RParen => write!(f, ")"),
            TokenValue::LBrace => write!(f, "{{"),
            TokenValue::RBrace => write!(f, "}}"),
            TokenValue::Semicolon => write!(f, ";"),
            TokenValue::Colon => write!(f, ":"),
            TokenValue::Arrow => write!(f, "->"),
            TokenValue::Assignment => write!(f, "="),
            TokenValue::PlusAssignment => write!(f, "+="),
            TokenValue::MinusAssignment => write!(f, "-="),
            TokenValue::StarAssignment => write!(f, "*="),
            TokenValue::SlashAssignment => write!(f, "/="),
            TokenValue::Or => write!(f, "||"),
            TokenValue::And => write!(f, "&&"),
            TokenValue::NotEqual => write!(f, "!="),
            TokenValue::Equal => write!(f, "=="),
            TokenValue::GreaterOrEqual => write!(f, ">="),
            TokenValue::Greater => write!(f, ">"),
            TokenValue::LessOrEqual => write!(f, "<="),
            TokenValue::Less => write!(f, "<"),
            TokenValue::Plus => write!(f, "+"),
            TokenValue::Minus => write!(f, "-"),
            TokenValue::Star => write!(f, "*"),
            TokenValue::Slash => write!(f, "/"),
            TokenValue::Comma => write!(f, ","),
            TokenValue::Not => write!(f, "!"),
            TokenValue::Type(ty) => write!(f, "{ty}"),
            TokenValue::Id(id) => write!(f, "{id}"),
            TokenValue::Literal(literal) => write!(f, "{literal}"),
            TokenValue::EOF => write!(f, "end of input"),
        }
    }
}
