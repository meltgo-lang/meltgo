use lempet::*;

pub fn lexer_test(input: &str) {
    let mut lex = LexerBuilder::new();
    lex.word("a").word("b").err_jmp(';').word(";").repeat("c");
    lex.run(input);
    println!("{:?}", lex.get_tokens());
}
