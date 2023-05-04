mod break_statement;
mod continue_statement;
mod declare_statement;
mod expr_statement;
mod function_statement;
mod if_statement;
mod return_statement;
mod while_statement;

use derive_enum_from_into::EnumTryInto;

use crate::{token::TokenValue, Error, TokenStream};

pub use self::{
    break_statement::BreakStatement,
    continue_statement::ContinueStatement,
    declare_statement::DeclareStatement,
    expr_statement::ExprStatement,
    function_statement::{FunctionArg, FunctionStatement},
    if_statement::{Branch, IfStatement},
    return_statement::ReturnStatement,
    while_statement::WhileStatement,
};

use super::Parse;

#[derive(EnumTryInto, PartialEq, Debug)]
pub enum Statement {
    Break(BreakStatement),
    Continue(ContinueStatement),
    Declare(DeclareStatement),
    Expr(ExprStatement),
    Function(FunctionStatement),
    If(IfStatement),
    Return(ReturnStatement),
    While(WhileStatement),
}

impl Parse for Statement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        let statement = {
            match token_stream.current().value {
                TokenValue::Break => Statement::Break(BreakStatement::parse(token_stream)?),
                TokenValue::Let => Statement::Declare(DeclareStatement::parse(token_stream)?),
                TokenValue::Fn => Statement::Function(FunctionStatement::parse(token_stream)?),
                TokenValue::Return => Statement::Return(ReturnStatement::parse(token_stream)?),
                TokenValue::While => Statement::While(WhileStatement::parse(token_stream)?),
                TokenValue::If => Statement::If(IfStatement::parse(token_stream)?),

                TokenValue::Continue => {
                    Statement::Continue(ContinueStatement::parse(token_stream)?)
                }

                _ => Statement::Expr(ExprStatement::parse(token_stream)?),
            }
        };

        Ok(statement)
    }
}
