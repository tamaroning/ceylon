use super::*;

#[test]
fn test_cursor() {
    let mut tokens = tokenize("// comment1\n// comment2\n/");
    assert_eq!(tokens.next().unwrap().kind, LineComment);
    assert_eq!(tokens.next().unwrap().kind, LineComment);
    assert_eq!(tokens.next().unwrap().kind, Slash);
    assert!(matches!(tokens.next(), None));
}
