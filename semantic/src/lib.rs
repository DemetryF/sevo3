use ast::{Semanticize, Statement};
use environment::Environment;
use error::Error;

pub mod ast;
pub mod error;

mod environment;

pub fn semanticize(statements: Vec<frontend::ast::Statement>) -> Result<Vec<Statement>, Error> {
    let mut env = Environment::default();

    statements
        .into_iter()
        .map(|stmt| stmt.semanticize(&mut env))
        .collect()
}
