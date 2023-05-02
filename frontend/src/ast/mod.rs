mod block;
mod expr;
mod operators;
mod statement;
mod utils;

use crate::{Error, TokenStream};

pub use self::{block::Block, expr::Expr, statement::*, utils::Id};
pub use crate::token::{Pos, Type};
pub use operators::{BinOp, UnOp};

pub trait Parse: Sized {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error>;
}
