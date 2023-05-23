use frontend::ast;

use crate::{
    ast::{Expr, FunctionId, Semanticize},
    environment::Environment,
    error::Error,
};

pub struct ReturnStatement {
    pub expr: Option<Expr>,
    pub function_ref: FunctionId,
}

impl Semanticize for ast::ReturnStatement {
    type Item = ReturnStatement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        let expr = self.expr.map(|expr| expr.semanticize(env)).transpose()?;

        let Some(function_ref) = env.functions.last() else {
            return Err(Error::return_outside_function());
        };

        Ok(ReturnStatement { expr, function_ref })
    }
}
