/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::num::{NonZeroU32, NonZeroUsize};

use colored::*;

#[derive(Debug, Clone, Copy)]
pub enum ErrorState {
    ImportantError,
    NormalError,
    Warning,
}

pub fn print_error(
    es: ErrorState,
    error_num: NonZeroU32,
    error_msg: &str,
    note: &str,
    file_path: &str,
    line: NonZeroUsize,
    column: NonZeroUsize,
) {
    match es {
        ErrorState::ImportantError => {
            println!(
                "{}: {}",
                format!("ERROR[IE{:0>4}]", error_num).red(),
                error_msg
            );
            println!(
                "  {} {}:{}:{}",
                "-->".bright_cyan(),
                file_path,
                line,
                column
            );
            println!("   {}", "|".bright_cyan());
            println!("   {} note: {}", "=".bright_cyan(), note);
        }
        ErrorState::NormalError => {
            println!(
                "{}: {}",
                format!("ERROR[NE{:0>4}]", error_num).red(),
                error_msg
            );
            println!(
                "  {} {}:{}:{}",
                "-->".bright_cyan(),
                file_path,
                line,
                column
            );
            println!("   {}", "|".bright_cyan());
            println!("   {} note: {}", "=".bright_cyan(), note);
        }
        ErrorState::Warning => {}
    }
}
