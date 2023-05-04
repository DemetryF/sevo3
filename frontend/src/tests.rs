#[allow(unused)]
use crate::{ast::*, token::Literal, token_stream::TokenStream};

#[allow(unused)]
macro_rules! int {
    ($value:literal) => {
        Expr::Atom(Atom::Literal(Literal::Integer($value)))
    };
}

#[allow(unused)]
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
macro_rules! prefix {
    ($Op:ident, $rhs:expr) => {
        Expr::Prefix {
            op: UnOp::$Op,
            rhs: Box::new($rhs),
        }
    };
}

macro_rules! parser_tests {
    (
        group $group:ident($Type:ident);
        $(
            $test_name:ident: $input:literal => $expected:expr,
        )*
    ) => {
        #[cfg(test)]
        pub mod $group {
            use super::*;

            $(
                #[test]
                fn $test_name() {
                    let mut token_stream = TokenStream::new($input).unwrap();

                    assert_eq!(
                        $Type::parse(&mut token_stream).unwrap(),
                        $Type::try_from($expected).unwrap()
                    );
                }
            )*
        }
    };
}

parser_tests![
    group expr(Expr);

    arithmetic:
        "-1 + 2 * 3" => infix!(
            prefix!(Neg, int!(1)),
            Add,
            infix!(int!(2), Mul, int!(3))
        ),

    comparison:
        "1 + 2 >= 3 - 4" => infix!(
            infix!(int!(1), Add, int!(2)),
            GE,
            infix!(int!(3), Sub, int!(4))
        ),

    logical:
        "!1 || 2 && 3" => infix!(
            prefix!(Not, int!(1)),
            Or,
            infix!(int!(2), And, int!(3))
        ),

    cast:
        "1 as f32" => Expr::Cast {
            lhs: Box::new(int!(1)),
            ty: Type::F32,
        },

    parenthesis:
        "1 * (2 + 3)" => infix!(
            int!(1),
            Mul,
            infix!(int!(2), Add, int!(3))
        ),
];
