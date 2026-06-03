use std::sync::{Arc, Mutex};

use nom::{
    IResult, Parser,
    bytes::{
        complete::{take_while, take_while1},
        tag,
    },
    character::complete::space1,
    combinator::{opt, recognize},
    sequence::pair,
};

use crate::mlw::mlw::{MLWFunction, MLWGrammarLeaf, MLWTypeVar};

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(|c: char| c.is_alphabetic() || c == '_'),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))
    .parse(input)
}

pub fn defvar(input: &str) -> IResult<&str, (&str, bool)> {
    let (input, _) = tag("let").parse(input)?;
    let (input, _) = space1(input)?;
    let (input, opt_mut) = opt((tag("mut"), space1)).parse(input)?;
    let (input, vname) = identifier(input)?;
    let is_mut = match opt_mut {
        Some(("mut", _)) => true,
        _ => false,
    };
    Ok((input, (vname, is_mut)))
}
