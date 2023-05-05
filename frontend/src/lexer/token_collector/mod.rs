mod number_collector;
mod special_collector;
mod word_collector;

use crate::{lexer::CodeStream, token::TokenValue};

pub use self::{
    number_collector::NumberCollector, special_collector::SpecialCollector,
    word_collector::WordCollector,
};

pub trait TokenCollector {
    fn try_collect(&mut self, code_stream: &mut CodeStream) -> Option<TokenValue>;
}
