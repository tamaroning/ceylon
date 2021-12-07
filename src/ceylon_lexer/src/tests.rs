use super::*;
use expect_test::{expect, Expect};

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();
    println!("{}", actual);
    expect.assert_eq(&actual)
}

#[test]
fn check_all_token() {
    check_lexing(
        "// comment\n ;,.(){}[]@#~?:$=!<>-&|+*/^%",
        expect![[r#"
        Token { kind: LineComment, len: 10 }
        Token { kind: Whitespace, len: 2 }
        Token { kind: Semi, len: 1 }
        Token { kind: Comma, len: 1 }
        Token { kind: Dot, len: 1 }
        Token { kind: OpenParen, len: 1 }
        Token { kind: CloseParen, len: 1 }
        Token { kind: OpenBrace, len: 1 }
        Token { kind: CloseBrace, len: 1 }
        Token { kind: OpenBracket, len: 1 }
        Token { kind: CloseBracket, len: 1 }
        Token { kind: At, len: 1 }
        Token { kind: Pound, len: 1 }
        Token { kind: Tilde, len: 1 }
        Token { kind: Question, len: 1 }
        Token { kind: Colon, len: 1 }
        Token { kind: Dollar, len: 1 }
        Token { kind: Eq, len: 1 }
        Token { kind: Bang, len: 1 }
        Token { kind: Lt, len: 1 }
        Token { kind: Gt, len: 1 }
        Token { kind: Minus, len: 1 }
        Token { kind: And, len: 1 }
        Token { kind: Or, len: 1 }
        Token { kind: Plus, len: 1 }
        Token { kind: Star, len: 1 }
        Token { kind: Slash, len: 1 }
        Token { kind: Caret, len: 1 }
        Token { kind: Percent, len: 1 }
        "#]],
    )
}

#[test]
fn smoke_test() {
    check_lexing(
        "// my source file\nfunc main() -> Void { println(\"Hello.\"); }\n",
        expect![[r#"
            Token { kind: LineComment, len: 17 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Gt, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 7 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 8 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}
