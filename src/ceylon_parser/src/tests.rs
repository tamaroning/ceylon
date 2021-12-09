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
fn test_parse_add() {
    check_parsing(
        "1 * 2 * 3",
        expect![[r#"
            Expr { kind: Binary(Mul, Expr { kind: Binary(Mul, Expr { kind: Literal(Int(1)), span: Span { start_pos: 0, len: 1 } }, Expr { kind: Literal(Int(2)), span: Span { start_pos: 4, len: 1 } }), span: Span { start_pos: 0, len: 5 } }, Expr { kind: Literal(Int(3)), span: Span { start_pos: 8, len: 1 } }), span: Span { start_pos: 0, len: 9 } }
        "#]],
    )
}
