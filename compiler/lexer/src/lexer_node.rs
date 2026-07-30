/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use popdack::lex::*;
use popdack::utils::*;

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
        let f = lstring(
            "let",
            lmany1(
                " ",
                land(
                    lreduce(
                        |s1, s2| s1 + s2,
                        lpredicate(
                            |c| c.is_alphabetic(),
                            lreduce(
                                |s1, s2| s1 + s2,
                                lfmany0(lpredicate(|c| c.is_alphanumeric(), lign())),
                            ),
                        ),
                    ),
                    lmany0(" ", lstring("=", lmany0(" ", lign()))),
                ),
            ),
        );
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Array(
                    vec.iter()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<String>>(),
                )
            }
            _ => LexResult::None,
        }
    }
}

#[lexer]
pub struct Use;
impl LexRule for Use {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let f = lstring(
            "use",
            lreduce(
                |s1, s2| s1 + s2,
                lmany1(
                    " ",
                    land(
                        lpredicate(
                            |c| c.is_alphabetic(),
                            lfmany0(lpredicate(|c| c.is_alphanumeric(), lign())),
                        ),
                        lfmany0(lmany0(
                            " ",
                            lstring(
                                "::",
                                lmany0(
                                    " ",
                                    lpredicate(
                                        |c| c.is_alphabetic(),
                                        lfmany0(lpredicate(|c| c.is_alphanumeric(), lign())),
                                    ),
                                ),
                            ),
                        )),
                    ),
                ),
            ),
        );
        match f(input.as_str()) {
            Ok((rest, vec)) => {
                *input = rest;
                LexResult::Array(
                    vec.iter()
                        .map(|s| s.replace(" ", "").to_string())
                        .collect::<Vec<String>>(),
                )
            }
            _ => LexResult::None,
        }
    }
}
