use crate::ast::Parse;
use crate::ast::{BinOp, Error, Id, TokenStream, Type, UnOp};
use crate::token::Literal;

pub enum Expr {
    Infix {
        lhs: Box<Self>,
        op: BinOp,
        rhs: Box<Self>,
    },
    Prefix {
        op: UnOp,
        rhs: Box<Self>,
    },
    Cast {
        lhs: Box<Self>,
        ty: Type,
    },
    Call {
        id: Id,
        args: Vec<Self>,
    },
    Atom(Atom),
}

pub enum Atom {
    Id(String),
    Literal(Literal),
}

impl Parse for Expr {
    fn parse(_token_stream: &mut TokenStream) -> Result<Self, Error> {
        todo!()
    }
}
