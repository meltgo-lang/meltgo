use popdack::lex::*;

use crate::lexer_node::*;

pub struct CargryLexer {
    lexer: Lexer,
}

impl LexerManager for CargryLexer {
    fn new() -> Self {
        let mut lex = Lexer::new();
        lex.add_rule(Let);
        lex.add_rule(Number);
        lex.add_rule(Ident);
        Self { lexer: lex }
    }
    fn run(&mut self, input: &str) {
        self.lexer.run(input);
    }
}
