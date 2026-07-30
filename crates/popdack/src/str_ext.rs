/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub fn skip(s: &String, i: usize) -> String {
    s.chars().skip(i).collect::<String>()
}

pub fn skip_while<F>(s: &String, f: F) -> String
where
    F: FnMut(&char) -> bool,
{
    s.chars().skip_while(f).collect::<String>()
}
