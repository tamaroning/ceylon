use super::*;
use expect_test::{expect, Expect};

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual)
}

#[test]
fn test_all_tokens() {
    check_lexing(
        "// comment\n ;,.(){}[]@#~?:$=!<>-&|+*/^%",
        expect![[r#"
            Token { kind: LineComment, span: Span { start_pos: 0, len: 10 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 2 } }
            Token { kind: Semi, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Comma, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Dot, span: Span { start_pos: 0, len: 1 } }
            Token { kind: OpenParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: CloseParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: OpenBrace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: CloseBrace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: OpenBracket, span: Span { start_pos: 0, len: 1 } }
            Token { kind: CloseBracket, span: Span { start_pos: 0, len: 1 } }
            Token { kind: At, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Pound, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Tilde, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Question, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Colon, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Dollar, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Eq, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Bang, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Lt, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Gt, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Minus, span: Span { start_pos: 0, len: 1 } }
            Token { kind: And, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Or, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Plus, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Star, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Slash, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Caret, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Percent, span: Span { start_pos: 0, len: 1 } }
        "#]],
    )
}

#[test]
fn test_textual_literal() {
    check_lexing(
        "'a''b''\\n''\\t''\\r'\"Hello\"\"//\"",
        expect![[r#"
            Token { kind: Literal { kind: Char { terminated: true } }, span: Span { start_pos: 0, len: 3 } }
            Token { kind: Literal { kind: Char { terminated: true } }, span: Span { start_pos: 0, len: 3 } }
            Token { kind: Literal { kind: Char { terminated: true } }, span: Span { start_pos: 0, len: 4 } }
            Token { kind: Literal { kind: Char { terminated: true } }, span: Span { start_pos: 0, len: 4 } }
            Token { kind: Literal { kind: Char { terminated: true } }, span: Span { start_pos: 0, len: 4 } }
            Token { kind: Literal { kind: Str { terminated: true } }, span: Span { start_pos: 0, len: 7 } }
            Token { kind: Literal { kind: Str { terminated: true } }, span: Span { start_pos: 0, len: 4 } }
        "#]],
    )
}

#[test]
fn test_unterminated_string_literal() {
    check_lexing(
        "\"",
        expect![[r#"
            Token { kind: Literal { kind: Str { terminated: false } }, span: Span { start_pos: 0, len: 1 } }
        "#]],
    );
}

#[test]
fn test_unterminated_char_literal() {
    check_lexing(
        "\'",
        expect![[r#"
            Token { kind: Literal { kind: Char { terminated: false } }, span: Span { start_pos: 0, len: 1 } }
        "#]],
    );
}

#[test]
fn test_numerical_literal() {
    check_lexing(
        "0 1 638462071 3.141592",
        expect![[r#"
            Token { kind: Literal { kind: Int }, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Literal { kind: Int }, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Literal { kind: Int }, span: Span { start_pos: 0, len: 9 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Literal { kind: Float }, span: Span { start_pos: 0, len: 8 } }
        "#]],
    )
}

#[test]
fn test_method_access() {
    check_lexing(
        "10.foo() 2.72.foo()",
        expect![[r#"
            Token { kind: Literal { kind: Int }, span: Span { start_pos: 0, len: 2 } }
            Token { kind: Dot, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Ident, span: Span { start_pos: 0, len: 3 } }
            Token { kind: OpenParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: CloseParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Literal { kind: Float }, span: Span { start_pos: 0, len: 4 } }
            Token { kind: Dot, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Ident, span: Span { start_pos: 0, len: 3 } }
            Token { kind: OpenParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: CloseParen, span: Span { start_pos: 0, len: 1 } }
        "#]],
    )
}

#[test]
fn smoke_test() {
    check_lexing(
        "// my source file\nfunc main() -> void { println(\"Hello.\"); }\n",
        expect![[r#"
            Token { kind: LineComment, span: Span { start_pos: 0, len: 17 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Ident, span: Span { start_pos: 0, len: 4 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Ident, span: Span { start_pos: 0, len: 4 } }
            Token { kind: OpenParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: CloseParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Minus, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Gt, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Ident, span: Span { start_pos: 0, len: 4 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: OpenBrace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Ident, span: Span { start_pos: 0, len: 7 } }
            Token { kind: OpenParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Literal { kind: Str { terminated: true } }, span: Span { start_pos: 0, len: 8 } }
            Token { kind: CloseParen, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Semi, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: CloseBrace, span: Span { start_pos: 0, len: 1 } }
            Token { kind: Whitespace, span: Span { start_pos: 0, len: 1 } }
        "#]],
    )
}

fn check_string_reader(src: &str, expect: Expect) {
    let mut tokens = Vec::new();
    let mut reader = StringReader::new(src);
    let mut token = reader.next_token();
    while token.kind != TokenKind::Eof {
        tokens.push(format!("{:?}\n", token));
        token = reader.next_token();
    }
    tokens.push(format!("{:?}\n", token));
    let actual: String = tokens.into_iter().collect();
    expect.assert_eq(&actual);
}

#[test]
fn test_string_reader() {
    check_string_reader(
        "// my source file\nfunc main() -> void { println(\"Hello.\"); }\n",
        expect![[r#"
            Token { kind: Ident, span: Span { start_pos: 18, len: 4 } }
            Token { kind: Ident, span: Span { start_pos: 23, len: 4 } }
            Token { kind: OpenParen, span: Span { start_pos: 27, len: 1 } }
            Token { kind: CloseParen, span: Span { start_pos: 28, len: 1 } }
            Token { kind: Minus, span: Span { start_pos: 30, len: 1 } }
            Token { kind: Gt, span: Span { start_pos: 31, len: 1 } }
            Token { kind: Keyword { kind: Void }, span: Span { start_pos: 33, len: 4 } }
            Token { kind: OpenBrace, span: Span { start_pos: 38, len: 1 } }
            Token { kind: Ident, span: Span { start_pos: 40, len: 7 } }
            Token { kind: OpenParen, span: Span { start_pos: 47, len: 1 } }
            Token { kind: Literal { kind: Str { terminated: true } }, span: Span { start_pos: 48, len: 8 } }
            Token { kind: CloseParen, span: Span { start_pos: 56, len: 1 } }
            Token { kind: Semi, span: Span { start_pos: 57, len: 1 } }
            Token { kind: CloseBrace, span: Span { start_pos: 59, len: 1 } }
            Token { kind: Eof, span: Span { start_pos: 61, len: 0 } }
        "#]],
    )
}

#[test]
fn test_keyword() {
    check_string_reader(
        "i64 u64 char str bool void if main",
        expect![[r#"
            Token { kind: Keyword { kind: I64 }, span: Span { start_pos: 0, len: 3 } }
            Token { kind: Keyword { kind: U64 }, span: Span { start_pos: 4, len: 3 } }
            Token { kind: Keyword { kind: Char }, span: Span { start_pos: 8, len: 4 } }
            Token { kind: Keyword { kind: Str }, span: Span { start_pos: 13, len: 3 } }
            Token { kind: Ident, span: Span { start_pos: 17, len: 4 } }
            Token { kind: Keyword { kind: Void }, span: Span { start_pos: 22, len: 4 } }
            Token { kind: Keyword { kind: If }, span: Span { start_pos: 27, len: 2 } }
            Token { kind: Ident, span: Span { start_pos: 30, len: 4 } }
            Token { kind: Eof, span: Span { start_pos: 34, len: 0 } }
        "#]],
    )
}
