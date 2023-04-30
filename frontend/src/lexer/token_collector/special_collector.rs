use crate::{
    lexer::{code_stream::CodeStream, TokenCollector},
    TokenValue,
};

pub struct SpecialCollector;
impl TokenCollector for SpecialCollector {
    fn try_collect(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        Self::double(code_stream).or(Self::single(code_stream))
    }
}

impl SpecialCollector {
    pub fn double(code_stream: &mut CodeStream) -> Option<TokenValue> {
        let value = match code_stream.slice_from_current(2) {
            ">=" => TokenValue::GreaterOrEqual,
            "<=" => TokenValue::LessOrEqual,
            "!=" => TokenValue::NotEqual,
            "==" => TokenValue::Equal,

            "+=" => TokenValue::PlusAssignment,
            "-=" => TokenValue::MinusAssignment,
            "*=" => TokenValue::StarAssignment,
            "/=" => TokenValue::SlashAssignment,

            "->" => TokenValue::Arrow,

            "||" => TokenValue::Or,
            "&&" => TokenValue::And,

            _ => return None,
        };

        code_stream.skip(2);

        Some(value)
    }

    pub fn single(code_stream: &mut CodeStream) -> Option<TokenValue> {
        let value = match code_stream.current() {
            ';' => TokenValue::Semicolon,
            ':' => TokenValue::Colon,
            ',' => TokenValue::Comma,
            '{' => TokenValue::LBrace,
            '}' => TokenValue::RBrace,
            '(' => TokenValue::LParen,
            ')' => TokenValue::RParen,
            '=' => TokenValue::Assignment,
            '>' => TokenValue::Greater,
            '<' => TokenValue::Less,
            '+' => TokenValue::Plus,
            '-' => TokenValue::Minus,
            '*' => TokenValue::Star,
            '/' => TokenValue::Slash,
            '!' => TokenValue::Not,

            _ => return None,
        };

        code_stream.skip(1);

        Some(value)
    }
}
