use crate::ast::{Block, Id, Parse, Type};
use crate::token::TokenValue;
use crate::{Error, TokenStream};

#[derive(PartialEq, Debug)]
pub struct FunctionStatement {
    pub id: Id,
    pub args: Vec<FunctionArg>,
    pub return_ty: Option<Type>,
    pub body: Block,
}

#[derive(PartialEq, Debug)]
pub struct FunctionArg {
    pub id: Id,
    pub ty: Type,
}

impl Parse for FunctionStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Id::parse(token_stream)?;
        let args = parse_function_args(token_stream)?;

        let return_ty = {
            if token_stream.try_consume(TokenValue::Arrow)? {
                Some(Type::parse(token_stream)?)
            } else {
                None
            }
        };

        let body = Block::parse(token_stream)?;

        Ok(FunctionStatement {
            id,
            args,
            return_ty,
            body,
        })
    }
}

fn parse_function_args(token_stream: &mut TokenStream) -> Result<Vec<FunctionArg>, Error> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen)? {
        return Ok(args);
    }

    args.push(FunctionArg::parse(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma)? {
        let arg = FunctionArg::parse(token_stream)?;

        args.push(arg)
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

impl Parse for FunctionArg {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let id = Id::parse(token_stream)?;
        token_stream.consume(TokenValue::Colon)?;
        let ty = Type::parse(token_stream)?;

        Ok(FunctionArg { id, ty })
    }
}
