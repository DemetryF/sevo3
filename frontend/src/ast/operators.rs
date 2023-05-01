use crate::token::{Token, TokenValue};
use crate::Error;

macro_rules! operators {
    (
        $(
            $GroupName:ident {
                $(
                    $OpName:ident: $ser:expr, $token_alt:pat, $power:expr;
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
                            $token_alt => Self::$OpName,
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
    AssignOp {
        Assign:     "=",    TokenValue::Assignment,         (2, 1);
        AddAssign:  "+=",   TokenValue::PlusAssignment,     (2, 1);
        SubAssign:  "-=",   TokenValue::MinusAssignment,    (2, 1);
        MulAssign:  "*=",   TokenValue::StarAssignment,     (2, 1);
        DivAssign:  "/=",   TokenValue::SlashAssignment,    (2, 1);
    },

    BinOp {

        Or:         "||",   TokenValue::Or,                 (3, 4);
        And:        "&&",   TokenValue::And,                (5, 6);

        EQ:         "==",   TokenValue::Equal,              (7, 8);
        NE:         "!=",   TokenValue::NotEqual,           (7, 8);
        GE:         ">=",   TokenValue::GreaterOrEqual,     (9, 10);
        GT:         ">",    TokenValue::Greater,            (9, 10);
        LE:         "<=",   TokenValue::LessOrEqual,        (9, 10);
        LT:         "<",    TokenValue::Less,               (9, 10);

        Add:        "+",    TokenValue::Plus,               (11, 12);
        Sub:        "-",    TokenValue::Minus,              (11, 12);
        Mul:        "*",    TokenValue::Star,               (13, 14);
        Div:        "/",    TokenValue::Slash,              (13, 14);

        Cast:       "as",   TokenValue::As,                 (15, 16);
    },

    UnOp {
        Not:        "!",  TokenValue::Not,                  (0, 15);
        Neg:        "-",  TokenValue::Minus,                (0, 15);
    }
];
