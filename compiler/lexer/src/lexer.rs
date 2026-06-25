use lempet::*;

pub fn l<'a>() -> LexerBuilder<'a> {
    let builder = lexer();
    builder.word("abc")
}

pub fn ident<'a>(funs: &mut LexerFunctionsBuffer<Predicate>) -> LexerBuilder<'a> {
    let builder = lexer();
    let builder2 = lexer();
    let builder2 = builder2.predicate(funs, Box::new(|c: char| c.is_alphabetic()));
    let builder = builder.repeat(builder2);
    builder
}
