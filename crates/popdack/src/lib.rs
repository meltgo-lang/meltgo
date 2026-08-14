/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod parser_lib;
mod parser_utils;

pub mod utils {
    pub use crate::parser_utils::*;
}

pub mod parser {
    pub use crate::parser_lib::*;
    pub use popdack_macro::mappinger;
}
