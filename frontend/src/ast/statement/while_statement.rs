use crate::ast::{Block, Expr, Parse};
use crate::token::TokenValue;
use crate::{Error, TokenStream};

#[derive(PartialEq, Debug)]
pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}

impl Parse for WhileStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::While)?;

        let condition = Expr::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        Ok(WhileStatement { condition, body })
    }
}
