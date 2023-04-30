use parse_int::parse;

use crate::{
    lexer::{CodeStream, TokenCollector},
    Literal, TokenValue,
};

const RADIX_PREFIX_LENGTH: usize = 2;

pub struct NumberCollector;
impl TokenCollector for NumberCollector {
    fn try_collect(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue> {
        if !Self::is_digit(code_stream, 10) {
            return None;
        }

        let start = code_stream.get_index();

        let ty = match code_stream.slice_from_current(RADIX_PREFIX_LENGTH) {
            "0b" => Self::prefixed(code_stream, 2),
            "0o" => Self::prefixed(code_stream, 8),
            "0x" => Self::prefixed(code_stream, 16),

            _ => Self::common_number(code_stream),
        };

        let end = code_stream.get_index();

        let buffer = code_stream.slice(start, end);
        let value = ty.parse(buffer);

        Some(TokenValue::Literal(value))
    }
}

impl NumberCollector {
    fn is_digit(code_stream: &CodeStream, radix: u32) -> bool {
        code_stream.current().is_digit(radix)
    }

    fn prefixed(code_stream: &mut CodeStream, radix: u32) -> NumberType {
        code_stream.skip(RADIX_PREFIX_LENGTH);

        Self::number_literal(code_stream, radix);

        NumberType::Integer
    }

    fn common_number(code_stream: &mut CodeStream) -> NumberType {
        Self::number_literal(code_stream, 10);

        let has_fraction = Self::fraction(code_stream);
        let has_exp = Self::exponential_part(code_stream);

        if has_fraction || has_exp {
            NumberType::Float
        } else {
            NumberType::Integer
        }
    }

    fn fraction(code_stream: &mut CodeStream) -> bool {
        if code_stream.check('.') {
            code_stream.consume();

            Self::number_literal(code_stream, 10);

            true
        } else {
            false
        }
    }

    fn exponential_part(code_stream: &mut CodeStream) -> bool {
        if code_stream.check('e') || code_stream.check('E') {
            code_stream.consume();

            if code_stream.check('-') || code_stream.check('+') {
                code_stream.consume();
            }

            Self::number_literal(code_stream, 10);

            true
        } else {
            false
        }
    }

    fn number_literal(code_stream: &mut CodeStream, radix: u32) {
        while !code_stream.is_eof()
            && (Self::is_digit(code_stream, radix) || code_stream.check('_'))
        {
            code_stream.consume();
        }
    }
}

#[derive(PartialEq)]
enum NumberType {
    Float,
    Integer,
}

impl NumberType {
    pub fn parse(self, str: &str) -> Literal {
        match self {
            Self::Float => {
                let value = parse::<f64>(str).unwrap();

                Literal::Float(value)
            }
            Self::Integer => {
                let value = parse::<i64>(str).unwrap();

                Literal::Integer(value)
            }
        }
    }
}
