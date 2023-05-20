mod block;
mod expr;
mod operators;
mod statement;

pub use frontend::ast::{Literal, Pos, Type};

pub use self::{block::*, expr::*, operators::*, statement::*};

use crate::{environment::Environment, error::Error};

pub trait Semanticize: Sized {
    type Item;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, Error>;
}
