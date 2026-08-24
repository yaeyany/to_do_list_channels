use crate::{commands::TDLCommands::AddTask, errors::TDLErrors::{self, CommandError}, input::user_input, task::TaskTitle};

pub enum TDLCommands {
    AddTask(TaskTitle),
    Quit
}

pub fn match_command(command: &str) -> Result<TDLCommands, TDLErrors>{
    match command {
        "add" => {
            println!("Please name your new task: ");
            let task_title = user_input()?.try_into()?;
            println!("Success");
            Ok(AddTask(task_title))
        },
        "quit" => {
            println!("Thank you for using this to do list");
            Ok(TDLCommands::Quit)
        },
        _ => Err(CommandError)
    }
}