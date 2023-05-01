use std::mem;

use crate::{lexer::Lexer, Error, Token, TokenValue};

pub struct TokenStream<'code> {
    lexer: Lexer<'code>,
    current: Token,
}

impl<'code> TokenStream<'code> {
    pub fn new(code: &'code str) -> Result<Self, Error> {
        let mut lexer = Lexer::new(code);

        let token_stream = Self {
            lexer: Lexer::new(code),
            current: lexer.next_token()?,
        };

        Ok(token_stream)
    }

    pub fn current(&self) -> &Token {
        &self.current
    }

    pub fn next(&mut self) -> Result<(), Error> {
        self.current = self.lexer.next_token()?;

        Ok(())
    }

    pub fn next_and_take(&mut self) -> Result<Token, Error> {
        let mut token = self.lexer.next_token()?;

        mem::swap(&mut self.current, &mut token);

        Ok(token)
    }

    pub fn check(&self, value: TokenValue) -> bool {
        self.current().value == value
    }

    pub fn consume(&mut self, value: TokenValue) -> Result<(), Error> {
        if self.check(value) {
            self.next()
        } else {
            self.unexpected_token()
        }
    }

    pub fn try_consume(&mut self, value: TokenValue) -> Result<bool, Error> {
        if self.check(value) {
            self.next()?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn unexpected_token(&mut self) -> Result<(), Error> {
        let token = self.next_and_take()?;
        let error = Error::unexpected_token(token);

        Err(error)
    }
}
