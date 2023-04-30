use std::fmt::{self, Display};

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

    Assignment,
    PlusAssignment,
    MinusAssignment,
    StarAssignment,
    SalshAssignment,

    Or,
    And,

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

    Not,

    // other
    Type(Type),
    Id(String),
    Literal(Literal),
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
            TokenValue::SalshAssignment => write!(f, "/="),
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
            TokenValue::Not => write!(f, "!"),
            TokenValue::Type(ty) => write!(f, "{ty}"),
            TokenValue::Id(id) => write!(f, "{id}"),
            TokenValue::Literal(literal) => write!(f, "{literal}"),
        }
    }
}

#[rustfmt::skip]
#[derive(strum_macros::Display)]
pub enum Type {
    U64,    I64,    F64,
    U32,    I32,    F32,
    U16,    I16,
    U8,     I8,     Bool,
}

pub enum Literal {
    Bool(bool),
    Integer(i64),
    Float(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Bool(bool) => write!(f, "{bool}"),
            Literal::Integer(int) => write!(f, "{int}"),
            Literal::Float(float) => write!(f, "{float}"),
        }
    }
}
