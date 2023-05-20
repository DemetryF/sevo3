use frontend::ast;

use crate::{
    ast::{Expr, Semanticize},
    environment::Environment,
    error::Error,
};

pub struct ExprStatement(pub Expr);

impl Semanticize for ast::ExprStatement {
    type Item = ExprStatement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        Ok(ExprStatement(self.0.semanticize(env)?))
    }
}
