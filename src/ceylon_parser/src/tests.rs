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
            Expr { kind: Literal(Literal { kind: Int(123456) }), span: Span { start_pos: 0, len: 6 } }
        "#]],
    )
}

#[test]
fn test_parse_float() {
    check_parsing(
        "3.141592",
        expect![[r#"
            Expr { kind: Literal(Literal { kind: Float(3.141592) }), span: Span { start_pos: 0, len: 8 } }
        "#]],
    )
}
