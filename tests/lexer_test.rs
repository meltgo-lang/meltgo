use lempet::*;

fn lex<'a>(input: &str) -> LexerBuilder<'a> {
    let mut lex = LexerBuilder::new();
    lex.word("a").word("b").err_jmp(';').word(";").repeat("c");
    lex.run(input);
    lex
}

#[test]
fn lexer_test() {
    assert_eq!(lex("ab;cc").get_tokens(), vec!["a", "b", ";", "cc"]);
}
