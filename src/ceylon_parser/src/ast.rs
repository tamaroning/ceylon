use ceylon_lexer::{LitKind, Span};

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
    Literal(Literal),
    Binary(BinOpKind, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind,
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
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}
