use crate::ast::Parse;
use crate::ast::{BinOp, Error, Id, TokenStream, Type, UnOp};
use crate::token::{Literal, TokenValue};

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
    Id(Id),
    Literal(Literal),
}

impl Parse for Expr {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, Error> {
        expr_bp(0, token_stream)
    }
}

fn expr_bp(current_bp: usize, token_stream: &mut TokenStream) -> Result<Expr, Error> {
    let mut lhs = {
        let token = token_stream.next_and_take()?;

        match token.value {
            TokenValue::Literal(literal) => Expr::Atom(Atom::Literal(literal)),

            TokenValue::Id(_) => {
                let id = Id::parse(token_stream)?;

                if token_stream.try_consume(TokenValue::LParen)? {
                    let mut args = Vec::new();

                    if !token_stream.try_consume(TokenValue::RParen)? {
                        args.push(Expr::parse(token_stream)?);

                        while token_stream.try_consume(TokenValue::Comma)? {
                            args.push(Expr::parse(token_stream)?);
                        }

                        token_stream.consume(TokenValue::Semicolon)?;
                    }

                    Expr::Call {
                        id,
                        args: Vec::new(),
                    }
                } else {
                    Expr::Atom(Atom::Id(id))
                }
            }

            TokenValue::LParen => {
                let expr = Expr::parse(token_stream)?;

                token_stream.consume(TokenValue::RParen)?;

                expr
            }

            _ => {
                if let Ok(op) = UnOp::try_from(&token.value) {
                    token_stream.next()?;

                    let (_, r_bp) = op.power();

                    let rhs = expr_bp(r_bp, token_stream)?;
                    let rhs = Box::new(rhs);

                    Expr::Prefix { op, rhs }
                } else {
                    return Err(Error::unexpected_token(token));
                }
            }
        }
    };

    while let Ok(op) = BinOp::try_from(&token_stream.current().value) {
        let (l_bp, r_bp) = op.power();

        if l_bp < current_bp {
            break;
        }

        token_stream.next()?;

        lhs = {
            let rhs = expr_bp(r_bp, token_stream)?;

            Expr::Infix {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        }
    }

    Ok(lhs)
}
