use std::num::{NonZeroU32, NonZeroUsize};
use std::sync::OnceLock;

use regex::Regex;

use actoa::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar, PseudoPointer};
use lexer::*;
use popdack::{Statement, StatementManager};
use psictre::ast::{ErrorBuf, Function, NodeBuf};
use suzlun_errors::*;

#[derive(Debug)]
pub enum MeltgoStatement {
    Package,
    Import,
    Let,
    Func,
    Comment,
    None,
}

impl Statement<MeltgoStatement> for MeltgoStatement {
    fn mapping(input: &str) -> MeltgoStatement {
        static FUNC_RE: OnceLock<Regex> = OnceLock::new();
        let func_regex = FUNC_RE.get_or_init(|| Regex::new(r"(pub\s+)?func").unwrap());
        match input {
            s if s.starts_with("package") => Self::Package,
            s if s.starts_with("import") => Self::Import,
            s if s.starts_with("let") => Self::Let,
            s if func_regex.is_match(s) => Self::Func,
            s if s.starts_with("//") || s.starts_with("/*") => Self::Comment,
            _ => Self::None,
        }
    }
}

fn main() {
    print_error(
        ErrorState::ImportantError,
        NonZeroU32::new(1).unwrap(),
        "a",
        "b",
        "src/main.suz",
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

}
