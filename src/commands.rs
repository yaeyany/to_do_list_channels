use std::sync::mpsc::Sender;

use crate::{errors::TDLErrors::{self, CommandError}, input::user_input};

pub enum TDLCommands {
    AddTask{
        task_title: String,
        response_sender: Sender<String>
    },
    Quit
}

pub fn match_command(command: &str, response_sender: Sender<String>) -> Result<TDLCommands, TDLErrors>{
    match command {
        "add" => {
            println!("Please name your new task: ");
            let task_title = user_input()?;
            Ok(TDLCommands::AddTask { 
                task_title: task_title, 
                response_sender
            })
        },
        "quit" => {
            println!("Thank you for using this to do list");
            Ok(TDLCommands::Quit)
        },
        _ => Err(CommandError)
    }
}