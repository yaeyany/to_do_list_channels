use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum TDLErrors {
    #[error("Invalid title. Can't be empty or longer than 50.")]
    TitleError,
    #[error("Failed to read input.")]
    InputError,
    #[error("Unknown command.")]
    CommandError,
}

pub fn handle_error(e: TDLErrors) {
    println!("{e}");
}
