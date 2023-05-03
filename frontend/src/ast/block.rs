use crate::ast::Parse;
use crate::token::TokenValue;
use crate::{Error, TokenStream};

use super::Statement;

pub struct Block {
    pub statements: Vec<Statement>,
}

impl Parse for Block {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let mut statements = Vec::new();

        token_stream.consume(TokenValue::LBrace)?;

        while !token_stream.try_consume(TokenValue::RBrace)? {
            let statement = Statement::parse(token_stream)?;

            statements.push(statement);
        }

        token_stream.consume(TokenValue::RBrace)?;

        Ok(Block { statements })
    }
}