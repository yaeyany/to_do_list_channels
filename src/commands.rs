use crate::errors::TDLErrors::{self, CommandError};

pub enum TDLCommands {
    AddTask,
    Quit
}

pub fn match_command(command: &str) -> Result<TDLCommands, TDLErrors>{
    match command {
        "add" => Ok(TDLCommands::AddTask),
        "quit" => Ok(TDLCommands::Quit),
        _ => Err(CommandError)
    }
}