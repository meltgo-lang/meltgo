use colored::*;

pub enum ErrorState {
    ImportantError,
    NormalError,
    Warning,
}

pub fn print_error(es: ErrorState, error_num: u32, error_msg: &str, note: &str) {
    match es {
        ErrorState::ImportantError => {
            println!(
                "{}: {}",
                format!("error[IE{:0>4}]", error_num).red(),
                error_msg
            );
        }
        ErrorState::NormalError => {
            println!("error[NE{:0>4}]: {}", error_num, error_msg);
        }
        ErrorState::Warning => {}
    }
}
