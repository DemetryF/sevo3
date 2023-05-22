use frontend::ast;

pub use ast::UnOp;

#[derive(Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,

    Or,
    And,

    EQ,
    NE,
    GE,
    GT,
    LE,
    LT,
}

#[derive(Clone, Copy)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl TryFrom<ast::BinOp> for BinOp {
    type Error = ();

    fn try_from(value: ast::BinOp) -> Result<Self, Self::Error> {
        match value {
            ast::BinOp::Or => Ok(Self::Or),
            ast::BinOp::And => Ok(Self::And),
            ast::BinOp::EQ => Ok(Self::EQ),
            ast::BinOp::NE => Ok(Self::NE),
            ast::BinOp::GE => Ok(Self::GE),
            ast::BinOp::GT => Ok(Self::GT),
            ast::BinOp::LE => Ok(Self::LE),
            ast::BinOp::LT => Ok(Self::LT),
            ast::BinOp::Add => Ok(Self::Add),
            ast::BinOp::Sub => Ok(Self::Sub),
            ast::BinOp::Mul => Ok(Self::Mul),
            ast::BinOp::Div => Ok(Self::Div),

            _ => return Err(()),
        }
    }
}

impl TryFrom<ast::BinOp> for AssignOp {
    type Error = ();

    fn try_from(value: ast::BinOp) -> Result<Self, Self::Error> {
        match value {
            ast::BinOp::Assign => Ok(Self::Assign),
            ast::BinOp::AddAssign => Ok(Self::AddAssign),
            ast::BinOp::SubAssign => Ok(Self::SubAssign),
            ast::BinOp::MulAssign => Ok(Self::MulAssign),
            ast::BinOp::DivAssign => Ok(Self::DivAssign),

            _ => Err(()),
        }
    }
}
