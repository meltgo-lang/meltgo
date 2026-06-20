use lempet::*;

pub fn lexer_test(input: &str) {
    let mut lex = Lexer::new();
    lex.word("abc").word("def");
    lex.excute(input);
    println!("{:?}", lex.get_tokens());
}
