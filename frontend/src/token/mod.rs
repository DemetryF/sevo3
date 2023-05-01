mod pos;
mod token_value;

use derive_more::Constructor;

pub use self::{
    pos::Pos,
    token_value::{Literal, TokenValue, Type},
};

#[derive(Constructor, Debug)]
pub struct Token {
    pub value: TokenValue,
    pub pos: Pos,
}
