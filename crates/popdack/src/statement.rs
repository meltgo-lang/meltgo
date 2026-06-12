use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    sync::{Arc, Mutex, OnceLock},
};

use regex::Regex;

use actoa::{MLWFunction, MLWGrammarLeaf, MLWTypeVar};
use psictre::ast::{Node, NodeBuf, Position, Ref};

#[derive(Debug)]
pub enum Statement {
    Package,
    Import,
    Let,
    Func,
    Comment,
    None,
}

impl Statement {
    pub fn mapping(input: &str) -> Statement {
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
