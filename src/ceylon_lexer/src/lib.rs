mod cursor;

#[cfg(test)]
mod tests;

use self::TokenKind::*;
use cursor::Cursor;

pub struct StringReader<'a> {
    pub src: &'a str,
    pos: usize,
    end_index: usize,
}

impl<'a> StringReader<'a> {
    pub fn new(src: &'a str) -> Self {
        StringReader {
            src,
            pos: 0,
            end_index: src.len(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            let text = &self.src[self.pos..self.end_index];
            if text.is_empty() {
                let span = Span {
                    start_pos: self.pos,
                    len: 0,
                };
                return Token::new(TokenKind::Eof, span);
            }

            let mut token = first_token(text);
            token.span.start_pos = self.pos;
            self.pos += token.span.len;

            match token.kind {
                TokenKind::Whitespace | TokenKind::LineComment => (),
                TokenKind::Ident => {
                    return self.ident_to_keyword(token);
                }
                _ => {
                    return token;
                }
            }
        }
    }

    fn ident_to_keyword(&self, t: Token) -> Token {
        debug_assert!(matches!(t.kind, TokenKind::Ident));
        let s = self.span_to_str(&t.span);
        let kw_kind = match s {
            "i64" => KwKind::I64,
            "u64" => KwKind::U64,
            "char" => KwKind::Char,
            "str" => KwKind::Str,
            "void" => KwKind::Void,
            "if" => KwKind::If,
            _ => {
                return t;
            }
        };
        Token {
            kind: TokenKind::Keyword { kind: kw_kind },
            span: t.span,
        }
    }

    pub fn span_to_str(&self, span: &Span) -> &str {
        &self.src[span.start_pos..span.start_pos + span.len]
    }

    pub fn quoted_to_str(&self, span: &Span) -> &str {
        debug_assert!(span.start_pos + 1 <= span.start_pos + span.len - 1);
        &self.src[span.start_pos + 1..span.start_pos + span.len - 1]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub start_pos: usize,
    pub len: usize,
}

impl Span {
    pub fn append(&self, s: Span) -> Self {
        let start_pos = std::cmp::min(self.start_pos, s.start_pos);
        let end_pos = std::cmp::max(self.start_pos + self.len, s.start_pos + s.len);
        Span {
            start_pos,
            len: end_pos - start_pos,
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
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
    /// "i32", "if", "while"
    Keyword { kind: KwKind },
    /// "12_u8", "1.0e-40", "b"123"". See `LitKind` for more details.
    Literal { kind: LitKind },
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
    /// EOF
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KwKind {
    /// "i64"
    I64,
    /// "u64"
    U64,
    /// "bool"
    Bool,
    /// "char"
    Char,
    /// "void"
    Void,
    /// "str"
    Str,
    /// "if"
    If,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LitKind {
    /// "12", "39"
    Int,
    /// "12.34", "5.0"
    Float,
    /// "'a'", "'\n'"
    Char { terminated: bool },
    /// ""abc"", ""abc"
    Str { terminated: bool },
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000D}' // \r
        | '\u{0020}' // space
    )
}

fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_id_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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
        input = &input[token.span.len..];
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
            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // Identifier (this should be checked after other variant that can
            // start as identifier).
            c if is_id_start(c) => self.ident(),

            // Numeric literal.
            c @ '0'..='9' => {
                let literal_kind = self.number(c);
                TokenKind::Literal { kind: literal_kind }
            }

            // One-symbol tokens.
            ';' => Semi,
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '@' => At,
            '#' => Pound,
            '~' => Tilde,
            '?' => Question,
            ':' => Colon,
            '$' => Dollar,
            '=' => Eq,
            '!' => Bang,
            '<' => Lt,
            '>' => Gt,
            '-' => Minus,
            '&' => And,
            '|' => Or,
            '+' => Plus,
            '*' => Star,
            '^' => Caret,
            '%' => Percent,

            // Character literal.
            '\'' => {
                let terminated = self.single_quoted_string();
                Literal {
                    kind: LitKind::Char { terminated },
                }
            }

            // String literal.
            '"' => {
                let terminated = self.double_quoted_string();
                Literal {
                    kind: LitKind::Str { terminated },
                }
            }

            _ => Unknown,
        };
        // NOTE: The tokenizer doesn't know the start position of the token.
        // For now tokenizer sets token.span.start_pos 0.
        let span = Span {
            start_pos: 0,
            len: self.len_consumed(),
        };
        Token::new(token_kind, span)
    }

    fn line_comment(&mut self) -> TokenKind {
        self.bump(); // The second slash of "// comment"
        self.eat_while(|c| c != '\n');
        LineComment
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        Whitespace
    }

    fn ident(&mut self) -> TokenKind {
        self.eat_while(is_id_continue);
        Ident
    }

    fn number(&mut self, _first_digit: char) -> LitKind {
        // first_digit is going to be used to parse hex ("0x4ef", "0x08")
        self.eat_decimal_digits();

        match self.first() {
            // Integer literals followed by dot can represent afield/method access
            '.' if !is_id_start(self.second()) => {
                self.bump(); // .
                if self.first().is_digit(10) {
                    self.eat_decimal_digits();
                }
                LitKind::Float
            }
            _ => LitKind::Int,
        }
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        while let '0'..='9' = self.first() {
            has_digits = true;
            self.bump();
        }
        has_digits
    }

    fn single_quoted_string(&mut self) -> bool {
        // Check if it's a one-symbol literal.
        if self.second() == '\'' && self.first() != '\\' {
            self.bump();
            self.bump();
            return true;
        }

        // Literal has more than one symbol.

        // Parse until either quotes are terminated or error is detected.
        loop {
            match self.first() {
                // Quotes are terminated, finish parsing.
                '\'' => {
                    self.bump();
                    return true;
                }
                // Probably beginning of the comment, which we don't want to include
                // to the error report.
                '/' => break,
                // Newline without following '\'' means unclosed quote, stop parsing.
                '\n' if self.second() != '\'' => break,
                // End of file, stop parsing.
                _ if self.is_eof() => break,
                // Escaped slash is considered one character, so bump twice.
                '\\' => {
                    self.bump();
                    self.bump();
                }
                // Skip the character.
                _ => {
                    self.bump();
                }
            }
        }
        // String was not terminated.
        false
    }

    /// Eats quoted string and returns true
    /// if string is terminated.
    /// Eats double-quoted string and returns true
    /// if string is terminated.
    fn double_quoted_string(&mut self) -> bool {
        while let Some(c) = self.bump() {
            match c {
                '"' => {
                    return true;
                }
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    // Bump again to skip escaped character.
                    self.bump();
                }
                _ => (),
            }
        }
        // End of file reached.
        false
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}
