use crate::{
    lexer::{CodeStream, TokenCollector},
    token::{Literal, TokenValue, Type},
};

pub struct WordCollector;
impl TokenCollector for WordCollector {
    fn try_collect(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_word_char(code_stream) {
            return None;
        }

        let buffer = Self::word_literal(code_stream);

        if let Ok(value) = Type::try_from(buffer) {
            return Some(TokenValue::Type(value));
        }

        let value = match buffer {
            "continue" => TokenValue::Continue,
            "return" => TokenValue::Return,
            "while" => TokenValue::While,
            "break" => TokenValue::Break,
            "else" => TokenValue::Else,
            "let" => TokenValue::Let,
            "mut" => TokenValue::Mut,
            "fn" => TokenValue::Fn,
            "if" => TokenValue::If,
            "as" => TokenValue::As,

            "true" => TokenValue::Literal(Literal::Bool(true)),
            "false" => TokenValue::Literal(Literal::Bool(false)),

            id => TokenValue::Id(id.into()),
        };

        Some(value)
    }
}

impl WordCollector {
    fn is_word_char(code_stream: &CodeStream) -> bool {
        code_stream.current().is_ascii_alphabetic()
            || code_stream.check('$')
            || code_stream.check('_')
    }

    fn word_literal<'code>(code_stream: &'code mut CodeStream) -> &'code str {
        let start = code_stream.get_index();

        while !code_stream.is_eof()
            && (Self::is_word_char(code_stream) || code_stream.current().is_alphanumeric())
        {
            code_stream.consume();
        }

        let end = code_stream.get_index();

        code_stream.slice(start, end)
    }
}
