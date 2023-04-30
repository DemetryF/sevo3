use crate::{lexer::Lexer, Literal, TokenValue, Type};

macro_rules! lexer_tests {
    (
        $(
            $test_name:ident(
                $input:literal
            ) {
                $(
                    $kind:expr,
                )*
            }
        )*
    ) => {
        $(
            #[test]
            fn $test_name() {
                let mut lexer = Lexer::new($input);

                $(
                    assert_eq!(
                        lexer.next_token().unwrap().value,
                        TokenValue::from($kind)
                    );
                )*
            }
        )*
    };
}

lexer_tests!(
    numbers("
        384_000     /* common integer */
        3.14159     /* common double  */
        6.67e-11    /* exponential notation (minus) */
        6.022e+23   /* exponential notation (plus)  */ 
        1E10        /* exponential notation (no)    */
    ") {
        Literal::Integer(384_000),
        Literal::Float(3.14159),
        Literal::Float(6.67e-11),
        Literal::Float(6.022e+23),
        Literal::Float(1E10),
    }

    boolean("
        true false
    ") {
        Literal::Bool(true),
        Literal::Bool(false),
    }

    keywords("
        continue return
        else fn while as
        if let mut break
    ") {
        TokenValue::Continue,
        TokenValue::Return,
        TokenValue::Else,
        TokenValue::Fn,
        TokenValue::While,
        TokenValue::As,
        TokenValue::If,
        TokenValue::Let,
        TokenValue::Mut,
        TokenValue::Break,
    }

    id("
        n1
        $n2
        _n3
    ") {
        TokenValue::Id("n1".into()),
        TokenValue::Id("$n2".into()),
        TokenValue::Id("_n3".into()),
    }

    specials("
        () {}
        ; : -> ,
        = += -= *= /=
        || && !
        != == >= > <= <
        + - * /
    ") {
        TokenValue::LParen,
        TokenValue::RParen,
        TokenValue::LBrace,
        TokenValue::RBrace,

        TokenValue::Semicolon,
        TokenValue::Colon,
        TokenValue::Arrow,
        TokenValue::Comma,

        TokenValue::Assignment,
        TokenValue::PlusAssignment,
        TokenValue::MinusAssignment,
        TokenValue::StarAssignment,
        TokenValue::SlashAssignment,

        TokenValue::Or,
        TokenValue::And,
        TokenValue::Not,

        TokenValue::NotEqual,
        TokenValue::Equal,
        TokenValue::GreaterOrEqual,
        TokenValue::Greater,
        TokenValue::LessOrEqual,
        TokenValue::Less,
        TokenValue::Plus,
        TokenValue::Minus,
        TokenValue::Star,
        TokenValue::Slash,
    }

    types("
        u64 u32 u16 u8
        i64 i32 i16 i8
        f64 f32
        bool 
    ") {
        Type::U64,
        Type::U32,
        Type::U16,
        Type::U8,

        Type::I64,
        Type::I32,
        Type::I16,
        Type::I8,

        Type::F64,
        Type::F32,

        Type::Bool,
    }
);
