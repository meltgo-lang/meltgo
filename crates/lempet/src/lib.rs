mod lempet;

pub use lempet::Lexer;

pub fn lexer() -> Lexer {
    Lexer::new()
}
