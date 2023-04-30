mod pos;
mod value;

pub use self::{
    pos::Pos,
    value::{Literal, TokenValue, Type},
};

pub struct Token {
    pub pos: Pos,
    pub value: TokenValue,
}
