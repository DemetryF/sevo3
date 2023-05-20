mod functions;
mod scope;
mod variables;

pub use self::{functions::Function, variables::Variable};
use self::{functions::Functions, variables::Variables};

#[derive(Default)]
pub struct Environment {
    pub functions: Functions,
    pub variables: Variables,
}
