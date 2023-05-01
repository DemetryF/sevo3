use crate::ast::{Parse, Pos, Type};
use crate::{token::TokenValue, Error, TokenStream};

pub struct Id {
    pub value: String,
    pub pos: Pos,
}

impl Parse for Id {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let token = token_stream.next_and_take()?;

        if let TokenValue::Id(value) = token.value {
            let id = Id {
                value,
                pos: token.pos,
            };

            Ok(id)
        } else {
            Err(Error::unexpected_token(token))
        }
    }
}

impl Parse for Type {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let token = token_stream.next_and_take()?;

        if let TokenValue::Type(ty) = token.value {
            Ok(ty)
        } else {
            Err(Error::unexpected_token(token))
        }
    }
}
