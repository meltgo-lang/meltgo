use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::NonZeroUsize,
    sync::{Arc, Mutex, OnceLock},
};

use regex::Regex;

use crate::{
    mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar},
    parser::ast::{Node, NodeBuf, Position, Ref},
};

pub enum Statement {
    Let,
}

impl Statement {
    pub fn mapping(input: &str) -> Statement {
        match input {
            s if s.starts_with("let") => Statement::Let,
            _ => panic!(),
        }
    }
}
