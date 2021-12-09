use super::*;
use expect_test::{expect, Expect};

fn check_parsing(src: &str, expect: Expect) {
    let actual: String = format!("{:?}\n", parse(src));
    expect.assert_eq(&actual)
}

#[test]
fn test_parse_integer() {
    check_parsing(
        "123456",
        expect![[r#"
            Expr { kind: Literal(Int(123456)), span: Span { start_pos: 0, len: 6 } }
        "#]],
    )
}

#[test]
fn test_parse_float() {
    check_parsing(
        "3.141592",
        expect![[r#"
            Expr { kind: Literal(Float(3.141592)), span: Span { start_pos: 0, len: 8 } }
        "#]],
    )
}

#[test]
fn test_parse_equality() {
    check_parsing(
        "1 + 2 * 3 == 6",
        expect![[r#"
            Expr { kind: Binary(Eq, Expr { kind: Binary(Add, Expr { kind: Literal(Int(1)), span: Span { start_pos: 0, len: 1 } }, Expr { kind: Binary(Mul, Expr { kind: Literal(Int(2)), span: Span { start_pos: 4, len: 1 } }, Expr { kind: Literal(Int(3)), span: Span { start_pos: 8, len: 1 } }), span: Span { start_pos: 4, len: 5 } }), span: Span { start_pos: 0, len: 9 } }, Expr { kind: Literal(Int(6)), span: Span { start_pos: 13, len: 1 } }), span: Span { start_pos: 0, len: 14 } }
        "#]],
    )
}
