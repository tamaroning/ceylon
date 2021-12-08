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
                let span = t.span;
                let s = self.span_to_str(&span);
                let n = s.parse::<u128>().unwrap();
                Literal {
                    kind: LiteralKind::Int(n),
                }
            }
            _ => panic!(),
        };
        Expr::new(ExprKind::Literal(lit), self.token.span)
    }
}
