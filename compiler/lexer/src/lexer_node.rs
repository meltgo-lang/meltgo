/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lewekk::lex::*;
use lewekk::utils::*;

#[lexer]
pub struct EndLine;
impl LexRule for EndLine {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let white_space = lreduce(
            |s1, s2| s1 + s2,
            lfmany0(lor(
                lstring(" ", lign()),
                lor(lstring("\n", lign()), lstring("\t", lign())),
            )),
        );
        let f = lconv(
            |vec| vec![vec[1].clone()],
            land(white_space.clone(), land(lstring(";", lign()), white_space)),
        );
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Scope;
impl LexRule for Scope {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lor(lstring("{", lign()), lstring("}", lign()));
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Paren;
impl LexRule for Paren {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lor(lstring("(", lign()), lstring(")", lign()));
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Number;
impl LexRule for Number {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lreduce(
            |s1, s2| s1 + s2,
            lfmany1(lpredicate(
                |c| c.is_numeric(),
                lor(
                    lstring(
                        ".",
                        lreduce(
                            |s1, s2| s1 + s2,
                            lfmany1(lpredicate(|c| c.is_numeric(), lign())),
                        ),
                    ),
                    lign(),
                ),
            )),
        );
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Ident;
impl LexRule for Ident {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lreduce(
            |s1, s2| s1 + s2,
            lfmany1(lpredicate(|c| c.is_alphabetic(), lign())),
        );
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Let;
impl LexRule for Let {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lstring("let", lign());
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Use;
impl LexRule for Use {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lstring("use", lign());
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Mod;
impl LexRule for Mod {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lstring("mod", lign());
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Some(vec[0].clone())
            }
            _ => LexResult::None,
        }
    }
}
