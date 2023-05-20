use frontend::ast;

use crate::{
    ast::{Expr, Semanticize, VariableId},
    environment::{Environment, Variable},
    error::Error,
};

pub struct DeclareStatement {
    pub id: VariableId,
    pub init_expr: Option<Expr>,
}

impl Semanticize for ast::DeclareStatement {
    type Item = DeclareStatement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        let variable = Variable {
            mutability: self.is_mut,
            defined_at: self.id.pos,
            ty: self.ty,
        };

        let id = env.variables.add_variable(self.id, variable)?;

        let init_expr = self
            .init_expr
            .map(|expr| expr.semanticize(env))
            .transpose()?;

        Ok(DeclareStatement { id, init_expr })
    }
}
