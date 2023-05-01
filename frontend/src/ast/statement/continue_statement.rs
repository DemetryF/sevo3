use crate::{ast::Parse, token::TokenValue, Error, TokenStream};

pub struct ContinueStatement;

impl Parse for ContinueStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::Continue)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ContinueStatement)
    }
}
