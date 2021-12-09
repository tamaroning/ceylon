use ceylon_lexer::Span;

use super::*;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub(crate) fn new(kind: ExprKind, span: Span) -> Self {
        Expr { kind, span }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(LiteralKind),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
}

#[derive(Debug)]
pub enum LiteralKind {
    Str(String),
    Char(char),
    Int(u128),
    Float(f64),
    Bool(bool),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
}

#[derive(Debug)]
pub enum UnOp {
    Not,
    Neg,
}
