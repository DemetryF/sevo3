use derive_more::Constructor;

use crate::Pos;

#[derive(Constructor, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedChar(char),
}
