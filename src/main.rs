mod mlw_test;

use mlw_test::*;

use std::num::{NonZeroU32, NonZeroUsize};

use popdack::lex::*;
use popdack::utils::*;

use cargry_errors::*;

use lexer::*;

fn main() {
    print_error(
        ErrorState::ImportantError,
        NonZeroU32::new(1).unwrap(),
        "a",
        "b",
        "src/main.crg",
        NonZeroUsize::new(1).unwrap(),
        NonZeroUsize::new(1).unwrap(),
    );
    //     let mut sm = StatementManager::<MeltgoStatement>::new(
    //         r"
    // package main;
    // import std::fmt;
    // ;
    // let a = 0;
    // func main() {
    //     let b = 1;
    // }",
    //     );
    //     sm.marking();
    //     println!("{:?}", sm);

    let mut lex = Lexer::new();
    lex.add_rule(EndLine);
    lex.add_rule(Use);
    lex.add_rule(Let);
    lex.add_rule(Number);
    lex.add_rule(Ident);
    lex.run("use mod  :: a::b;use std::fmt;");
    println!("tokens: {:?}", lex.get_tokens());

    let f = lfmany1(lpredicate(|ch| *ch == 'a', lign()));

    match f("aaaa") {
        Ok(vec) => println!("{:?}", vec),
        Err(e) => println!("{}", e),
    }

    // g();
}
