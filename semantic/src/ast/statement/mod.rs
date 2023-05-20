mod declare_statement;
mod expr_statement;
mod function_statement;
mod if_statement;
mod return_statement;
mod while_statement;

use frontend::ast;

pub use ast::{BreakStatement, ContinueStatement};

use crate::environment::Environment;

pub use self::{
    declare_statement::DeclareStatement,
    expr_statement::ExprStatement,
    function_statement::FunctionStatement,
    if_statement::{Branch, IfStatement},
    return_statement::ReturnStatement,
    while_statement::WhileStatement,
};

use super::Semanticize;

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

impl Semanticize for ast::Statement {
    type Item = Statement;

    fn semanticize(self, env: &mut Environment) -> Result<Self::Item, crate::error::Error> {
        match self {
            ast::Statement::Break(break_stmt) => Ok(Statement::Break(break_stmt)),
            ast::Statement::Continue(continue_stmt) => Ok(Statement::Continue(continue_stmt)),

            ast::Statement::While(while_stmt) => Ok(Statement::While(while_stmt.semanticize(env)?)),
            ast::Statement::Expr(expr_stmt) => Ok(Statement::Expr(expr_stmt.semanticize(env)?)),
            ast::Statement::If(if_stmt) => Ok(Statement::If(if_stmt.semanticize(env)?)),

            ast::Statement::Declare(declare_stmt) => {
                Ok(Statement::Declare(declare_stmt.semanticize(env)?))
            }

            ast::Statement::Function(function_stmt) => {
                Ok(Statement::Function(function_stmt.semanticize(env)?))
            }

            ast::Statement::Return(return_stmt) => {
                Ok(Statement::Return(return_stmt.semanticize(env)?))
            }
        }
    }
}
