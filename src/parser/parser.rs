use std::sync::{Arc, Mutex};

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

use crate::parser::ast::{MeltgoNode, MeltgoNodeBuf, Ref};

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(|c: char| c.is_alphabetic() || c == '_'),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))
    .parse(input)
}

fn white_space0(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c == ' ').parse(input)
}

fn white_space1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == ' ').parse(input)
}

pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

pub fn number<'src>(input: &'src str, buf: &mut MeltgoNodeBuf<'src>) -> IResult<&'src str, usize> {
    let (input, str_num) = take_while(is_digit).parse(input)?;

    match str_num.parse::<i32>() {
        Ok(i) => Ok((input, buf.push(MeltgoNode::Number { value: i }))),
        Err(_) => fail().parse(input),
    }
}

pub fn add_op<'src>(input: &'src str, buf: &mut MeltgoNodeBuf<'src>) -> IResult<&'src str, usize> {
    let (input, l) = number(input, buf)?;
    let (input, _) = white_space0(input)?;
    let (input, _) = tag("+").parse(input)?;
    let (input, _) = white_space0(input)?;
    let (input, r) = number(input, buf)?;

    Ok((
        input,
        buf.push(MeltgoNode::AddOp {
            l: Ref::new(l),
            r: Ref::new(r),
        }),
    ))
}

pub fn defvar<'src>(input: &'src str, buf: &mut MeltgoNodeBuf<'src>) -> IResult<&'src str, usize> {
    let (input, _) = tag("let").parse(input)?;
    let (input, _) = white_space1(input)?;
    let (input, opt_mut) = opt(pair(tag("mut"), space1)).parse(input)?;
    let is_mut = match opt_mut {
        Some(("mut", _)) => true,
        _ => false,
    };
    let (input, vname) = identifier(input)?;
    let (input, _) = white_space0(input)?;
    let (input, _) = tag("=").parse(input)?;
    let (input, _) = white_space0(input)?;
    let (input, expr) = add_op(input, buf)?;

    Ok((
        input,
        buf.push(MeltgoNode::Let {
            vname: vname,
            is_mut: is_mut,
            expr: Ref::new(expr),
        }),
    ))
}
