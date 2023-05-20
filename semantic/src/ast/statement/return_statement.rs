use frontend::ast;

use crate::{
    ast::{Expr, Semanticize},
    environment::Environment,
    error::Error,
};

pub struct ReturnStatement {
    pub expr: Option<Expr>,
}

impl Semanticize for ast::ReturnStatement {
    type Item = ReturnStatement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        let expr = self.expr.map(|expr| expr.semanticize(env)).transpose()?;

        Ok(ReturnStatement { expr })
    }
}
