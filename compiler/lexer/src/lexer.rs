/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lewekk::lex::*;

use crate::lexer_node::*;

pub struct CargryLexer {
    lexer: Lexer,
}

impl LexerManager for CargryLexer {
    fn new() -> Self {
        let mut lex = Lexer::new(EndLine);
        lex.add_rule(Scope);
        lex.add_rule(Paren);
        lex.add_rule(Use);
        lex.add_rule(Mod);
        lex.add_rule(Let);
        lex.add_rule(Number);
        lex.add_rule(Ident);
        Self { lexer: lex }
    }

    fn run(&mut self, input: &str) {
        self.lexer.run(input);
    }

    fn get_tokens(&self) -> &Vec<String> {
        self.lexer.get_tokens()
    }

    fn get_rules(&self) -> &Vec<(Box<dyn LexRule>, usize)> {
        self.lexer.get_rules()
    }
}
