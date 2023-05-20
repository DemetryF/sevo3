use frontend::ast;

use crate::{
    ast::{Block, Expr, Semanticize},
    environment::Environment,
    error::Error,
};

pub struct IfStatement {
    pub condition: Expr,
    pub body: Block,
    pub elseif: Vec<Branch>,
    pub else_body: Option<Block>,
}

pub struct Branch {
    pub condition: Expr,
    pub body: Block,
}

impl Semanticize for ast::IfStatement {
    type Item = IfStatement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        let condition = self.condition.semanticize(env)?;
        let body = self.body.semanticize(env)?;

        let elseif = self
            .elseif
            .into_iter()
            .map(|branch| branch.semanticize(env))
            .collect::<Result<Vec<_>, _>>()?;

        let else_body = self
            .else_body
            .map(|body| body.semanticize(env))
            .transpose()?;

        Ok(IfStatement {
            condition,
            body,
            elseif,
            else_body,
        })
    }
}

impl Semanticize for ast::Branch {
    type Item = Branch;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        let condition = self.condition.semanticize(env)?;
        let body = self.body.semanticize(env)?;

        Ok(Branch { condition, body })
    }
}
