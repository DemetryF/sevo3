use crate::ast::*;
use crate::int;
use crate::TokenStream;

#[test]
pub fn sample() {
    let mut token_stream = TokenStream::new("if 1 { 2; }").unwrap();
    let stmt = IfStatement::parse(&mut token_stream).unwrap();

    let expected = IfStatement {
        condition: int!(1),
        body: Block {
            statements: vec![Statement::Expr(ExprStatement(int!(2)))],
        },
        elseif: vec![],
        else_body: None,
    };

    assert_eq!(stmt, expected);
}

#[test]
pub fn with_else() {
    let code = "
            if 1 { 
                2;
            } else { 
                3; 
            }";

    let mut token_stream = TokenStream::new(code).unwrap();
    let stmt = IfStatement::parse(&mut token_stream).unwrap();

    let expected = IfStatement {
        condition: int!(1),
        body: Block {
            statements: vec![Statement::Expr(ExprStatement(int!(2)))],
        },
        elseif: vec![],
        else_body: Some(Block {
            statements: vec![Statement::Expr(ExprStatement(int!(3)))],
        }),
    };

    assert_eq!(stmt, expected)
}

#[test]
pub fn elseif() {
    let code = "
            if 1 { 
                2; 
            } else if 3 { 
                4; 
            }";

    let mut token_stream = TokenStream::new(code).unwrap();
    let stmt = IfStatement::parse(&mut token_stream).unwrap();

    let expected = IfStatement {
        condition: int!(1),
        body: Block {
            statements: vec![Statement::Expr(ExprStatement(int!(2)))],
        },
        elseif: vec![Branch {
            condition: int!(3),
            body: Block {
                statements: vec![Statement::Expr(ExprStatement(int!(4)))],
            },
        }],
        else_body: None,
    };

    assert_eq!(stmt, expected);
}

#[test]
pub fn elseif_else() {
    let code = "
            if 1 { 
                2; 
            } else if 3 { 
                4; 
            } else {
                5;
            }";

    let mut token_stream = TokenStream::new(code).unwrap();
    let stmt = IfStatement::parse(&mut token_stream).unwrap();

    let expected = IfStatement {
        condition: int!(1),
        body: Block {
            statements: vec![Statement::Expr(ExprStatement(int!(2)))],
        },
        elseif: vec![Branch {
            condition: int!(3),
            body: Block {
                statements: vec![Statement::Expr(ExprStatement(int!(4)))],
            },
        }],
        else_body: Some(Block {
            statements: vec![Statement::Expr(ExprStatement(int!(5)))],
        }),
    };

    assert_eq!(stmt, expected);
}
