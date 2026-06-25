use lempet::*;

#[test]
fn err_jmp_test() {
    let builder = lexer();
    let mut funs = lexer_fun();
    let mut builder = builder
        .word("a")
        .word("b")
        .err_jmp(";")
        .word(";")
        .repeat_str("c");
    let result = builder.run("ab;cc", &mut funs);
    result.set_tokens();
    assert_eq!(result.get_tokens(), vec!["a", "b", ";", "cc"]);
}

#[test]
fn predicate_test() {
    let builder = lexer();
    let mut funs = lexer_fun();
    let mut builder = builder
        .predicate(&mut funs, Box::new(|c: char| c.is_alphabetic()))
        .word(";");
    let result = builder.run("a;", &mut funs);
    result.set_tokens();
    assert_eq!(result.get_tokens(), vec!["a", ";"]);
}

#[test]
fn call_test() {
    let mut funs = lexer_fun();
    let builder = lexer();
    let builder2 = lexer();
    let builder = builder.word("a");
    let builder2 = builder2.word("b").word("c");
    let mut lexer = builder.call(builder2).word("d");
    let result = lexer.run("abcd", &mut funs);
    result.set_tokens();
    assert_eq!(result.get_tokens(), vec!["a", "b", "c", "d"]);
}

#[test]
fn repeat_call_test() {
    let mut funs = lexer_fun();
    let builder = lexer();
    let builder2 = lexer();

    let builder2 = builder2.predicate(&mut funs, Box::new(|c: char| c.is_alphabetic()));
    let mut lexer = builder.repeat(builder2);

    let result = lexer.run("abcd", &mut funs);
    result.set_tokens();
    assert_eq!(result.get_tokens(), vec!["abcd"]);
}
