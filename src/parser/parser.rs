use std::{
    fmt::Alignment::Left,
    sync::{Arc, Mutex},
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

use crate::mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar};

#[derive(Debug)]
pub struct Ref {
    pub ptr: usize,
}

impl Ref {
    pub fn new(ptr: usize) -> Self {
        Self { ptr: ptr }
    }
}

#[derive(Debug)]
pub enum MeltgoNode<'a> {
    Number {
        value: i32,
    },
    Let {
        vname: &'a str,
        is_mut: bool,
        expr: Ref,
    },
}

#[derive(Debug)]
pub struct NodeBuf<'a> {
    pub buf: Vec<MeltgoNode<'a>>,
}

impl<'a> NodeBuf<'a> {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }

    pub fn push(&mut self, node: MeltgoNode<'a>) -> usize {
        let size = self.buf.len();
        self.buf.push(node);
        size
    }
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(|c: char| c.is_alphabetic() || c == '_'),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))
    .parse(input)
}

pub fn number<'src>(input: &'src str, buf: &mut NodeBuf<'src>) -> IResult<&'src str, usize> {
    let (input, str_num) = digit1().parse(input)?;
    match str_num.parse::<i32>() {
        Ok(i) => Ok((input, buf.push(MeltgoNode::Number { value: i }))),
        Err(_) => fail().parse(input),
    }
}

pub fn defvar<'src>(input: &'src str, buf: &mut NodeBuf<'src>) -> IResult<&'src str, usize> {
    let (input, _) = tag("let").parse(input)?;
    let (input, _) = space1(input)?;
    let (input, opt_mut) = opt((tag("mut"), space1)).parse(input)?;
    let (input, vname) = identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("=").parse(input)?;
    let (input, _) = space0(input)?;
    let (input, expr) = number(input, buf)?;
    let is_mut = match opt_mut {
        Some(("mut", _)) => true,
        _ => false,
    };
    Ok((
        input,
        buf.push(MeltgoNode::Let {
            vname,
            is_mut,
            expr: Ref::new(expr),
        }),
    ))
}
