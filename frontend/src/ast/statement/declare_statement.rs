use crate::ast::{Expr, Id, Parse, Type};
use crate::token::TokenValue;
use crate::{Error, TokenStream};

pub struct DeclareStatement {
    pub id: Id,
    pub is_mut: bool,
    pub ty: Type,
    pub init_expr: Option<Expr>,
}

impl Parse for DeclareStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::Let)?;

        let id = Id::parse(token_stream)?;
        let is_mut = token_stream.try_consume(TokenValue::Mut)?;

        token_stream.consume(TokenValue::Colon)?;

        let ty = Type::parse(token_stream)?;

        let init_expr = {
            if token_stream.try_consume(TokenValue::Assignment)? {
                let expr = Expr::parse(token_stream)?;

                Some(expr)
            } else {
                None
            }
        };

        Ok(DeclareStatement {
            id,
            is_mut,
            ty,
            init_expr,
        })
    }
}
