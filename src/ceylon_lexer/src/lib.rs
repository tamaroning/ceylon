mod cursor;

#[cfg(test)]
mod tests;

use self::TokenKind::*;
use cursor::Cursor;
use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Multi-char tokens:
    /// "// comment"
    LineComment,
    /// Any whitespace characters sequence.
    Whitespace,
    /// "ident"
    Ident,
    /// "12_u8", "1.0e-40", "b"123"". See `LiteralKind` for more details.
    Literal { kind: LiteralKind },
    // One-char tokens:
    /// ";"
    Semi,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
    /// "@"
    At,
    /// "#"
    Pound,
    /// "~"
    Tilde,
    /// "?"
    Question,
    /// ":"
    Colon,
    /// "$"
    Dollar,
    /// "="
    Eq,
    /// "!"
    Bang,
    /// "<"
    Lt,
    /// ">"
    Gt,
    /// "-"
    Minus,
    /// "&"
    And,
    /// "|"
    Or,
    /// "+"
    Plus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "^"
    Caret,
    /// "%"
    Percent,
    /// Unknown token, not expected by the lexer
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    /// "12", "39"
    Int,
    /// "12.34", "5.0"
    Float,
    /// "'a'", "'\n'"
    Char { terminated: bool },
    /// ""abc"", ""abc"
    Str { terminated: bool },
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000D}' // \r
        | '\u{0020}' // space
    )
}

/// Parses the first token from the provided input string.
pub fn first_token(input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }
        let token = first_token(input);
        input = &input[token.len..];
        Some(token)
    })
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            // Slash or comment.
            '/' => match self.first() {
                '/' => self.line_comment(),
                _ => Slash,
            },
            _ => panic!("Unexpected character: {}", first_char),
        };
        Token::new(token_kind, self.len_consumed())
    }

    fn line_comment(&mut self) -> TokenKind {
        self.bump(); // The second slash of "// comment"
        self.eat_while(|c| c != '\n');
        self.bump(); // '\n'
        LineComment
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}
