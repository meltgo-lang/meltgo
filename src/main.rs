mod mlw_test;

use mlw_test::*;

use std::num::{NonZeroU32, NonZeroUsize};
use std::sync::OnceLock;

use regex::Regex;

use actoa::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer};
use cargry_errors::*;
use lexer::*;
use popdack::*;
use psictre::ast::{ErrorBuf, Function, NodeBuf};

#[lexer]
struct Number;
impl LexRule for Number {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let mut res = String::new();
        while let Some(c) = input.chars().next() {
            if c.is_numeric() {
                *input = input.chars().skip(1).collect::<String>();
                res.push(c);
            } else {
                break;
            }
        }
        if res.is_empty() {
            None
        } else {
            let tmp = input.chars().skip(res.len()).collect::<String>();
            println!("{}", res);
            Some((LexReturn::Some(res), tmp))
        }
    }
}

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
    lex.add_rule(Number);
    lex.run("123");
    println!("tokens: {:?}", lex.get_tokens());

    g();
}
