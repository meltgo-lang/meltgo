use popdack::lex::*;

#[derive(Clone)]
pub enum Direction {
    RIght,
    Left,
    RightLeft,
}

#[derive(Clone)]
pub struct ParsePrefix {
    pub prefix: i32,
    pub direction: Direction,
}

pub trait ContextDicide {
    fn decide_context(&self, lexer: &Lexer) -> ParsePrefix;
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        lexer,
    }
}
