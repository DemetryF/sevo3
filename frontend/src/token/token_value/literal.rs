use std::fmt;

use crate::token::TokenValue;

#[derive(PartialEq, Debug)]
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

impl From<Literal> for TokenValue {
    fn from(value: Literal) -> Self {
        TokenValue::Literal(value)
    }
}
