/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::marker::PhantomData;

use lewekk::lex::Lexer;

use crate::utils::{ParserMappinger, ParserVariant};

pub struct Parser<T1: ParserMappinger<T1, T2> + ParserVariant<T1>, T2> {
    pmap: PhantomData<T1>,
    node: Vec<T2>,
    lex: Lexer,
}
impl<T1, T2> Parser<T1, T2>
where
    T1: ParserMappinger<T1, T2> + ParserVariant<T1>,
{
    pub fn new(lex: Lexer) -> Self {
        Self {
            pmap: PhantomData.clone(),
            node: vec![],
            lex,
        }
    }

    pub fn run(&mut self) -> Result<&Vec<T2>, String> {
        let mut tokens = self
            .lex
            .get_rules()
            .iter()
            .map(|(x, _)| T1::variant(x))
            .collect::<Vec<T1>>();
        while tokens.len() > 0 {
            let value = tokens.get(0).unwrap();
            let res = T1::mapping(value)(&tokens);
            match res {
                Ok((new_node, new_tokens)) => {
                    self.node.push(new_node);
                    tokens = new_tokens;
                }
                Err(e) => {
                    Err(e)?;
                }
            }
        }
        Ok(&self.node)
    }
}
