#[allow(unused)]
use crate::{
    ast::{Atom, Expr, Literal},
    TokenStream,
};

#[allow(unused)]
#[macro_export]
macro_rules! int {
    ($value:literal) => {
        Expr::Atom(Atom::Literal(Literal::Integer($value)))
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! infix {
    ($lhs:expr, $Op:ident, $rhs:expr) => {
        Expr::Infix {
            lhs: Box::new($lhs),
            op: BinOp::$Op,
            rhs: Box::new($rhs),
        }
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! prefix {
    ($Op:ident, $rhs:expr) => {
        Expr::Prefix {
            op: UnOp::$Op,
            rhs: Box::new($rhs),
        }
    };
}
