use crate::ast::{Block, Expr, Parse};
use crate::token::TokenValue;
use crate::{Error, TokenStream};

#[derive(PartialEq, Debug)]
pub struct IfStatement {
    pub condition: Expr,
    pub body: Block,
    pub elseif: Vec<Branch>,
    pub else_body: Option<Block>,
}

#[derive(PartialEq, Debug)]
pub struct Branch {
    pub condition: Expr,
    pub body: Block,
}

impl Parse for IfStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::If)?;

        let condition = Expr::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        let mut elseif = Vec::new();
        let mut else_body = None;

        // println!("<if>{}", token_stream.current().value);

        while token_stream.try_consume(TokenValue::Else)? {
            if token_stream.try_consume(TokenValue::If)? {
                let condition = Expr::parse(token_stream)?;
                let body = Block::parse(token_stream)?;

                let branch = Branch { condition, body };

                elseif.push(branch);
            } else {
                else_body = Some(Block::parse(token_stream)?);

                break;
            }
        }

        Ok(IfStatement {
            condition,
            body,
            elseif,
            else_body,
        })
    }
}
