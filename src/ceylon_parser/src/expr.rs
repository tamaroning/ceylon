use super::*;
use ast::*;
use ceylon_lexer::LitKind;

impl Parser<'_> {
    pub(crate) fn parse_expression(&mut self) -> Expr {
        self.parse_operator_expression()
    }

    /// Parse operator expression
    // Operator precedence is as follows:
    // unary+,-  >  *,/,% > binary+,-  >  <,>,<=,>=  >  ==,!= 
    fn parse_operator_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_relational();
        loop {
            let binop = match self.token.kind {
                TokenKind::EqEq => BinOp::Eq,
                TokenKind::BangEq => BinOp::Ne,
                _ => return expr,
            };
            // Eat a operator
            self.bump();
            let oprand = self.parse_relational();
            let span = expr.span.append(oprand.span);
            expr = Expr::new(
                ExprKind::Binary(binop, Box::new(expr), Box::new(oprand)),
                span,
            );
        }
        expr
    }

    fn parse_relational(&mut self) -> Expr {
        let mut expr = self.parse_add();
        loop {
            let binop = match self.token.kind {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::LtEq => BinOp::Le,
                TokenKind::GtEq => BinOp::Ge,
                _ => return expr,
            };
            // Eat a operator
            self.bump();
            let oprand = self.parse_add();
            let span = expr.span.append(oprand.span);
            expr = Expr::new(
                ExprKind::Binary(binop, Box::new(expr), Box::new(oprand)),
                span,
            );
        }
        expr
    }

    fn parse_add(&mut self) -> Expr {
        let mut expr = self.parse_mul();
        loop {
            let binop = match self.token.kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => return expr,
            };
            // Eat a operator
            self.bump();
            let oprand = self.parse_mul();
            let span = expr.span.append(oprand.span);
            expr = Expr::new(
                ExprKind::Binary(binop, Box::new(expr), Box::new(oprand)),
                span,
            );
        }
        expr
    }

    fn parse_mul(&mut self) -> Expr {
        let mut expr = self.parse_unary();
        loop {
            let binop = match self.token.kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => return expr,
            };
            // Eat a operator
            self.bump();
            let oprand = self.parse_unary();
            let span = expr.span.append(oprand.span);
            expr = Expr::new(
                ExprKind::Binary(binop, Box::new(expr), Box::new(oprand)),
                span,
            );
        }
        expr
    }

    fn parse_unary(&mut self) -> Expr {
        match self.token.kind {
            TokenKind::Plus => {
                self.bump();
            }
            TokenKind::Minus => {
                todo!()
            }
            TokenKind::Bang => {
                todo!()
            }
            _ => (),
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        let expr;
        match self.token.kind {
            TokenKind::Literal { kind: k } => {
                expr = self.to_ast_literal(self.token);
                self.bump();
            }
            _ => unimplemented!(),
        }
        expr
    }

    fn to_ast_literal(&self, t: Token) -> Expr {
        debug_assert!(matches!(t.kind, TokenKind::Literal { .. }));
        let lit = match t.kind {
            TokenKind::Literal { kind: LitKind::Int } => {
                let s = self.reader.span_to_str(&t.span);
                let n = s.parse::<u128>().unwrap();
                LiteralKind::Int(n)
            }
            TokenKind::Literal {
                kind: LitKind::Float,
            } => {
                let s = self.reader.span_to_str(&t.span);
                let f = s.parse::<f64>().unwrap();
                LiteralKind::Float(f)
            }
            TokenKind::Literal {
                kind: LitKind::Char { terminated: termi },
            } => {
                if !termi {
                    panic!("Not terminated");
                }
                let s = self.reader.quoted_to_str(&t.span);
                let c = unescape(s).chars().nth(0).unwrap();
                LiteralKind::Char(c)
            }
            TokenKind::Literal {
                kind: LitKind::Str { terminated: termi },
            } => {
                if !termi {
                    panic!("Not terminated");
                }
                let s = self.reader.quoted_to_str(&t.span);
                LiteralKind::Str(unescape(s))
            }
            _ => unreachable!(),
        };
        Expr::new(ExprKind::Literal(lit), self.token.span)
    }
}

fn unescape(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\\t")
        .replace("\\\"", "\"")
        .replace("\\\'", "\'")
}
