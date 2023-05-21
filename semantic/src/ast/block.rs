use crate::{environment::Environment, error::Error};

use super::{Semanticize, Statement};

pub struct Block {
    pub statements: Vec<Statement>,
}

impl Semanticize for frontend::ast::Block {
    type Item = Block;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        env.variables.fork();

        let statements = self
            .statements
            .into_iter()
            .map(|body| body.semanticize(env))
            .collect::<Result<Vec<_>, _>>()?;

        env.variables.exit();

        Ok(Block { statements })
    }
}
