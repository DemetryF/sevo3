use crate::{ast::Parse, token::TokenValue, Error, TokenStream};

pub struct BreakStatement;

impl Parse for BreakStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::Break)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(BreakStatement)
    }
}
