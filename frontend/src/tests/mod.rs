#[allow(unused)]
use crate::{ast::*, int, TokenStream};

#[cfg(test)]
mod expr;
#[cfg(test)]
mod if_statement;
mod utils;

#[test]
pub fn while_stmt() {
    let code = "
        while 1 {
            2;
        }";

    let mut token_stream = TokenStream::new(code).unwrap();
    let stmt = WhileStatement::parse(&mut token_stream).unwrap();

    let expected = WhileStatement {
        condition: int!(1),
        body: Block {
            statements: vec![Statement::Expr(ExprStatement(int!(2)))],
        },
    };

    assert_eq!(stmt, expected);
}
