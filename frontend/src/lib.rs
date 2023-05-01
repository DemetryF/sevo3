pub mod ast;
pub mod error;
pub mod token;

mod lexer;
mod token_stream;

pub use error::{Error, ErrorKind};

use token::{Literal, Pos, Token, TokenValue, Type};
use token_stream::TokenStream;
