mod lempet;

pub use lempet::LexerBuilder;

pub fn lexer<'a>() -> LexerBuilder<'a> {
    LexerBuilder::<'a>::new()
}
