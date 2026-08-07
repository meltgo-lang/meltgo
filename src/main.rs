/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::num::{NonZeroU32, NonZeroUsize};

use popdack::lex::*;

use cargry_errors::*;

use lexer::*;

fn main() {
    // error-emitter test
    print_error(
        ErrorState::ImportantError,
        NonZeroU32::new(1).unwrap(),
        "a",
        "b",
        "src/main.crg",
        NonZeroUsize::new(1).unwrap(),
        NonZeroUsize::new(1).unwrap(),
    );

    // lexer test
    let mut lex = Lexer::new();
    lex.add_rule(EndLine);
    lex.add_rule(Use);
    lex.add_rule(Let);
    lex.add_rule(Number);
    lex.add_rule(Ident);
    lex.run("use mod  :: a::b;use std::fmt;");
    println!("tokens: {:?}", lex.get_tokens());
    println!("rule  : {:?}", lex.get_rules());
}
