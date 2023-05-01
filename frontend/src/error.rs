use derive_more::Constructor;

use crate::{Pos, Token, TokenValue};

#[derive(Constructor, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar(char),
    UnexpectedToken(TokenValue),
}

impl Error {
    pub fn unexpected_token(token: Token) -> Self {
        let kind = ErrorKind::UnexpectedToken(token.value);

        Error::new(kind, token.pos)
    }
}
