use crate::{
    ast::{Block, FunctionId, Semanticize},
    environment::{Environment, Function, Variable},
    error::Error,
};

pub struct FunctionStatement {
    pub id: FunctionId,
    pub body: Block,
}

impl Semanticize for frontend::ast::FunctionStatement {
    type Item = FunctionStatement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        env.variables.fork();

        let args = self
            .args
            .into_iter()
            .map(|arg| {
                let variable = Variable {
                    mutability: true,
                    defined_at: arg.id.pos,
                    ty: arg.ty,
                };

                env.variables.add_argument(arg.id, variable)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let function = Function {
            return_ty: self.return_ty,
            defined_at: self.id.pos,
            args,
        };

        let id = env.functions.set(self.id, function)?;
        let body = self.body.semanticize(env)?;

        env.variables.exit();

        Ok(FunctionStatement { id, body })
    }
}
