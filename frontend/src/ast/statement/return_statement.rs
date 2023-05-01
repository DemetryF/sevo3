use crate::ast::{Expr, Parse};
use crate::token::TokenValue;
use crate::{Error, TokenStream};

pub struct ReturnStatement {
    pub expr: Option<Expr>,
}

impl Parse for ReturnStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::Return)?;

        let expr = {
            if token_stream.check(TokenValue::Semicolon) {
                None
            } else {
                Some(Expr::parse(token_stream)?)
            }
        };

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ReturnStatement { expr })
    }
}
