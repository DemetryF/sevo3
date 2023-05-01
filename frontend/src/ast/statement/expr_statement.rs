use crate::ast::{Expr, Parse};
use crate::{Error, TokenStream};

pub struct ExprStatement(Expr);

impl Parse for ExprStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let expr = Expr::parse(token_stream)?;

        Ok(ExprStatement(expr))
    }
}
