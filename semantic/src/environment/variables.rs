use std::rc::Rc;

use frontend::ast;

use crate::{
    ast::{Pos, Type, VariableId},
    error::Error,
};

use super::scope::Scope;

pub struct Variable {
    pub mutability: bool,
    pub defined_at: Pos,
    pub ty: Type,
}

#[derive(Default)]
pub struct Variables {
    data: Vec<Scope<VariableId>>,
}

impl Variables {
    pub fn fork(&mut self) {
        self.data.push(Scope::default());
    }

    pub fn exit(&mut self) {
        self.data.pop();
    }

    pub fn add_variable(&mut self, id: ast::Id, variable: Variable) -> Result<VariableId, Error> {
        self.add(id, variable, |id, var| {
            Error::variable_redefinition(id, var.defined_at)
        })
    }

    pub fn add_argument(&mut self, id: ast::Id, variable: Variable) -> Result<VariableId, Error> {
        self.add(id, variable, |id, _| Error::duplicate_function_args(id))
    }

    fn add(
        &mut self,
        id: ast::Id,
        variable: Variable,
        error: impl FnOnce(ast::Id, VariableId) -> Error,
    ) -> Result<VariableId, Error> {
        if let Some(variable) = self.current().get(&id.value).clone() {
            return Err(error(id, variable));
        }

        let variable_id = Rc::new(variable);

        self.current_mut().add(id.value, variable_id.clone());

        Ok(variable_id)
    }

    pub fn get(&self, id: &ast::Id) -> Option<VariableId> {
        self.data
            .iter()
            .rev()
            .find_map(|scope| scope.get(&id.value))
    }

    fn current(&self) -> &Scope<VariableId> {
        self.data.last().unwrap()
    }

    fn current_mut(&mut self) -> &mut Scope<VariableId> {
        self.data.last_mut().unwrap()
    }
}
