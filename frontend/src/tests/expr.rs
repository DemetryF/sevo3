use crate::ast::*;
use crate::TokenStream;
use crate::{infix, int, prefix};

macro_rules! expr_tests {
    (
        $(
            $test_name:ident: $input:literal => $expected:expr,
        )*
    ) => {
            $(
                #[test]
                fn $test_name() {
                    let mut token_stream = TokenStream::new($input).unwrap();

                    assert_eq!(
                        Expr::parse(&mut token_stream).unwrap(),
                        Expr::try_from($expected).unwrap()
                    );
                }
            )*
    };
}

expr_tests![
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
