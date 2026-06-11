use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    sync::{Arc, Mutex, OnceLock},
};

use nom::{
    Err, IResult, Parser,
    bytes::{
        complete::{take_while, take_while1},
        tag,
    },
    character::{complete::space1, digit1, streaming::space0},
    combinator::{fail, opt, recognize},
    sequence::pair,
};
use regex::Regex;

use crate::{
    mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar},
    parser::ast::{Node, NodeBuf, Position, Ref},
};

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

    pub fn parse(&self, input: &str) -> IResult<&str, &str> {
        match &self {
            Self::Let => Ok(("", "")),
        }
    }
}
