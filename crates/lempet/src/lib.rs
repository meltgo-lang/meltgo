mod lempet;

pub use lempet::{LexerBuilder, LexerFunctionsBuffer, Predicate};

pub fn lexer<'a>() -> LexerBuilder<'a> {
    LexerBuilder::<'a>::new()
}

pub fn lexer_fun() -> LexerFunctionsBuffer<dyn Fn(char) -> bool> {
    LexerFunctionsBuffer::new()
}
