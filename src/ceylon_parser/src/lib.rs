extern crate ceylon_lexer;

mod ast;
mod expr;

#[cfg(test)]
mod tests;

use ceylon_lexer::{Span, StringReader, Token, TokenKind};

pub struct Parser<'a> {
    /// The current token.
    token: Token,
    /// The previous token.
    prev_token: Token,

    reader: StringReader<'a>,
}

// For now parses only expressions
pub(crate) fn parse(src: &str) -> ast::Expr {
    let mut parser = Parser::new(src);
    parser.parse_expression()
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut reader = StringReader::new(s);

        Parser {
            token: reader.next_token(),
            prev_token: Token::new(
                TokenKind::Unknown,
                Span {
                    start_pos: 0,
                    len: 0,
                },
            ),
            reader,
        }
    }

    fn bump(&mut self) {
        self.prev_token = self.token;
        self.token = self.reader.next_token();
    }
}
