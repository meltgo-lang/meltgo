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
                format!("error[IE{:0>4}]", error_num).red(),
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
                format!("error[NE{:0>4}]", error_num).red(),
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
