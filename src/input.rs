use std::io;

use crate::errors::TDLErrors::{self, InputError, TitleError};

//User input
pub fn user_input() -> Result<String, TDLErrors>{
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Err(InputError);
    }
    Ok(input.trim().to_string())
}

// Title validation, no longer than 50 and not empty
pub fn title_validate(title: String) -> Result<String, TDLErrors> {
    if title.len() > 50 || title.is_empty(){
        Err(TitleError)
    } else {
        Ok(title)
    }
}