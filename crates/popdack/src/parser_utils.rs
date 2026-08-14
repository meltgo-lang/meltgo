/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lewekk::lex::LexRule;

pub trait ParserVariant<T> {
    fn variant(input: &Box<dyn LexRule>) -> T;
}
pub trait ParserMappinger<T1, T2> {
    fn mapping(input: &T1) -> impl Fn(&Vec<T1>) -> Result<(T2, Vec<T1>), String>;
}
