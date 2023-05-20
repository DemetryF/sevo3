use std::rc::Rc;

use frontend::ast;

use crate::{
    ast::{FunctionId, Pos, Type, VariableId},
    error::Error,
};

use super::scope::Scope;

#[derive(Clone)]
pub struct Function {
    pub return_ty: Option<Type>,
    pub defined_at: Pos,
    pub args: Vec<VariableId>,
}

#[derive(Default)]
pub struct Functions {
    data: Scope<FunctionId>,
}

impl Functions {
    pub fn get(&self, id: &ast::Id) -> Option<FunctionId> {
        self.data.get(&id.value)
    }

    pub fn set(&mut self, id: ast::Id, function: Function) -> Result<FunctionId, Error> {
        if let Some(function) = self.get(&id) {
            return Err(Error::function_redefinition(id, function.defined_at));
        }

        let function_id = Rc::new(function);

        self.data.add(id.value, function_id.clone());

        Ok(function_id)
    }
}
