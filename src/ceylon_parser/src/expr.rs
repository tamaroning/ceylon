use super::*;
use ast::*;
use ceylon_lexer::LitKind;

impl Parser<'_> {
    pub(crate) fn parse_expression(&mut self) -> Expr {
        self.parse_operator_expression()
    }

    fn parse_operator_expression(&mut self) -> Expr {
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
                Literal {
                    kind: LiteralKind::Int(n),
                }
            }
            TokenKind::Literal {
                kind: LitKind::Float,
            } => {
                let s = self.reader.span_to_str(&t.span);
                let f = s.parse::<f64>().unwrap();
                Literal {
                    kind: LiteralKind::Float(f),
                }
            }
            TokenKind::Literal {
                kind: LitKind::Char { terminated: termi },
            } => {
                if !termi {
                    panic!("Not terminated");
                }
                let s = self.reader.quoted_to_str(&t.span);
                let c = unescape(s).chars().nth(0).unwrap();
                Literal {
                    kind: LiteralKind::Char(c),
                }
            }
            TokenKind::Literal {
                kind: LitKind::Str { terminated: termi },
            } => {
                if !termi {
                    panic!("Not terminated");
                }
                let s = self.reader.quoted_to_str(&t.span);
                Literal {
                    kind: LiteralKind::Str(unescape(s)),
                }
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
