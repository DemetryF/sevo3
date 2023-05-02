use crate::token::{Token, TokenValue};
use crate::Error;

macro_rules! operators {
    (
        $(
            $GroupName:ident {
                $(
                    $OpName:ident: $ser:expr, $token_alt:ident, $power:expr;
                )*
            }
        ),*
    ) => {
        $(
            #[derive(Debug, PartialEq, Clone)]
            pub enum $GroupName {
                $(
                    $OpName,
                )*
            }

            impl TryFrom<Token> for $GroupName {
                type Error = Error;

                fn try_from(token: Token) -> Result<Self, Self::Error> {
                    let op = match token.value {
                        $(
                            TokenValue::$token_alt => Self::$OpName,
                        )*

                        _ => return Err(Error::unexpected_token(token)),
                    };

                    Ok(op)
                }
            }

            impl $GroupName {
                pub fn power(&self) -> (usize, usize) {
                    match self {
                        $(
                            Self::$OpName => $power,
                        )*
                    }
                }
            }

            impl std::fmt::Display for $GroupName {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    match self {
                        $(
                            Self::$OpName => write!(f, $ser),
                        )*
                    }
                }
            }
        )*
    };
}

operators![
    BinOp {
        Assign:     "=",    Assignment,         (2, 1);
        AddAssign:  "+=",   PlusAssignment,     (2, 1);
        SubAssign:  "-=",   MinusAssignment,    (2, 1);
        MulAssign:  "*=",   StarAssignment,     (2, 1);
        DivAssign:  "/=",   SlashAssignment,    (2, 1);

        Or:         "||",   Or,                 (3, 4);
        And:        "&&",   And,                (5, 6);

        EQ:         "==",   Equal,              (7, 8);
        NE:         "!=",   NotEqual,           (7, 8);
        GE:         ">=",   GreaterOrEqual,     (9, 10);
        GT:         ">",    Greater,            (9, 10);
        LE:         "<=",   LessOrEqual,        (9, 10);
        LT:         "<",    Less,               (9, 10);

        Add:        "+",    Plus,               (11, 12);
        Sub:        "-",    Minus,              (11, 12);
        Mul:        "*",    Star,               (13, 14);
        Div:        "/",    Slash,              (13, 14);

        Cast:       "as",   As,                 (15, 16);
    },

    UnOp {
        Not:        "!",    Not,                (0, 15);
        Neg:        "-",    Minus,              (0, 15);
    }
];
