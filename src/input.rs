use std::io;

use crate::{errors::TDLErrors::{self, InputError, InvalidTaskId, InvalidTaskState, TitleError}, task::{TaskId, TaskState::{self, Done, InProgress, ToDo}}};

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

pub fn check_id(id: String) -> Result<TaskId, TDLErrors> {
    match id.parse::<u32>() {
        Ok(id) => Ok(id.try_into()?),
        Err(_) => Err(InvalidTaskId),
    }
}

pub fn check_state(state: String) -> Result<TaskState, TDLErrors> {
    match state.to_lowercase().as_str() {
        "todo" | "to do" => Ok(ToDo),
        "inprogress" | "in progress" => Ok(InProgress),
        "done" => Ok(Done),
        _ => Err(InvalidTaskState)
    }
}