use thiserror::{self, Error};

#[derive(Error, Debug)]
pub enum TDLErrors {
    #[error("Invalid title. Can't be empty or longer than 50.")]
    TitleError,
    #[error("Failed to read input.")]
    InputError,
    #[error("Unknown command.")]
    CommandError,
    #[error("Task not found")]
    InvalidTaskIdLookup,
    #[error("Task ID must be >=0 and within the list")]
    InvalidTaskId,
    #[error("Invalid state")]
    InvalidTaskState,
}

pub fn handle_error(e: TDLErrors) {
    println!("{e}");
}
