use frontend::ast;

use crate::{
    ast::{Block, Expr, Semanticize},
    environment::Environment,
    error::Error,
};

pub struct WhileStatement {
    pub condition: Expr,
    pub body: Block,
}

impl Semanticize for ast::WhileStatement {
    type Item = WhileStatement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        let condition = self.condition.semanticize(env)?;
        let body = self.body.semanticize(env)?;

        Ok(WhileStatement { condition, body })
    }
}
