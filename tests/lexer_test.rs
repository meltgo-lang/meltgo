use lempet::*;

#[test]
fn err_jmp_test() {
    let mut builder = LexerBuilder::new();
    let result = builder
        .word("a")
        .word("b")
        .err_jmp(';')
        .word(";")
        .repeat("c")
        .run("ab;cc");
    assert_eq!(result.get_tokens(), vec!["a", "b", ";", "cc"]);
}

#[test]
fn predicate_test() {
    let mut builder = LexerBuilder::new();
    let result = builder
        .predicate(|c| c.is_alphabetic())
        .word(";")
        .run("a;");
    assert_eq!(result.get_tokens(), vec!["a", ";"]);
}

#[test]
fn call_test() {
    let mut builder = LexerBuilder::new();
    let mut builder2 = LexerBuilder::new();
    let lexer = builder
        .word("a");
    let lexer2 = builder2
        .word("b")
        .word("c");
    let lexer = lexer
        .call(lexer2)
        .word("d");
    let result = lexer.run("abcd");
    assert_eq!(result.get_tokens(), vec!["a", "b", "c", "d"]);
}
