use crate::ast::{Expr, Parse};
use crate::token::TokenValue;
use crate::{Error, TokenStream};

#[derive(PartialEq, Debug)]
pub struct ExprStatement(pub Expr);

impl Parse for ExprStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let expr = Expr::parse(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement(expr))
    }
}
