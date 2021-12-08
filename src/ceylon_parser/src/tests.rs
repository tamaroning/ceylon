use super::*;
use expect_test::{expect, Expect};

#[test]
fn test_parse_integer() {
    expect![[r#"
        Expr { kind: Literal(Literal { kind: Int(50) }), span: Span { start_pos: 0, len: 2 } }
    "#]]
    .assert_eq(&format!("{:?}\n", parse("50")));
}
