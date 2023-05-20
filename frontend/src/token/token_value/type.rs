use crate::token::TokenValue;

#[rustfmt::skip]
#[derive(strum_macros::Display, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Type {
    U64,    I64,    F64,
    U32,    I32,    F32,
    U16,    I16,
    U8,     I8,     Bool,
}

impl TryFrom<&str> for Type {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "bool" {
            return Ok(Self::Bool);
        }

        let ty = match value {
            "bool" => Self::Bool,

            "u64" => Self::U64,
            "u32" => Self::U32,
            "u16" => Self::U16,
            "u8" => Self::U8,

            "i64" => Self::I64,
            "i32" => Self::I32,
            "i16" => Self::I16,
            "i8" => Self::I8,

            "f64" => Self::F64,
            "f32" => Self::F32,

            _ => return Err(()),
        };

        Ok(ty)
    }
}

impl From<Type> for TokenValue {
    fn from(value: Type) -> Self {
        TokenValue::Type(value)
    }
}

impl Type {
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Type::U64
                | Type::I64
                | Type::U32
                | Type::I32
                | Type::U16
                | Type::I16
                | Type::U8
                | Type::I8
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::F64 | Type::F32)
    }
}
