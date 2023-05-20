use std::rc::Rc;

use frontend::ast;

use crate::{
    environment::{Environment, Function, Variable},
    error::Error,
};

use super::{AssignOp, BinOp, Literal, Semanticize, Type, UnOp};

pub enum Expr {
    Infix {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Prefix {
        op: UnOp,
        rhs: Box<Expr>,
    },
    Assign {
        id: VariableId,
        op: AssignOp,
        value: Box<Expr>,
    },
    Cast {
        value: Box<Expr>,
        ty: Type,
    },
    Call {
        id: FunctionId,
        args: Vec<Expr>,
    },
    Atom(Atom),
}

pub enum Atom {
    Id(VariableId),
    Literal(Literal),
}

pub type VariableId = Rc<Variable>;
pub type FunctionId = Rc<Function>;

impl Semanticize for ast::Expr {
    type Item = Expr;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error> {
        match self {
            ast::Expr::Infix { lhs, op, rhs } => {
                if let Ok(op) = AssignOp::try_from(op) {
                    let ast::Expr::Atom(ast::Atom::Id(id)) = *lhs else {
                        return Err(Error::unexpected_assignment());
                    };

                    let Some(variable_id) = env.variables.get(&id) else {
                        return Err(Error::non_existent_variable(id));
                    };

                    let value = Box::new(rhs.semanticize(env)?);

                    Ok(Expr::Assign {
                        id: variable_id,
                        op,
                        value,
                    })
                } else if let Ok(op) = BinOp::try_from(op) {
                    let rhs = Box::new(rhs.semanticize(env)?);
                    let lhs = Box::new(lhs.semanticize(env)?);

                    Ok(Expr::Infix { lhs, op, rhs })
                } else {
                    unreachable!()
                }
            }

            ast::Expr::Prefix { op, rhs } => {
                let rhs = Box::new(rhs.semanticize(env)?);

                Ok(Expr::Prefix { op, rhs })
            }

            ast::Expr::Cast { lhs, ty } => {
                let value = Box::new(lhs.semanticize(env)?);

                Ok(Expr::Cast { value, ty })
            }

            ast::Expr::Call { id, args } => {
                let Some(function_id) = env.functions.get(&id) else {
                    return Err(Error::non_existent_function(id));
                };

                let args = args
                    .into_iter()
                    .map(|arg| arg.semanticize(env))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Expr::Call {
                    id: function_id,
                    args,
                })
            }

            ast::Expr::Atom(atom) => match atom {
                ast::Atom::Id(id) => {
                    let Some(id) = env.variables.get(&id) else {
                        return Err(Error::non_existent_variable(id));
                    };

                    Ok(Expr::Atom(Atom::Id(id)))
                }

                ast::Atom::Literal(literal) => Ok(Expr::Atom(Atom::Literal(literal))),
            },
        }
    }
}
