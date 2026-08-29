use std::sync::mpsc::Sender;

use crate::{errors::TDLErrors::{self, CommandError}, input::{check_id, check_state, input_confirmation, title_validate, user_input}, task::{self, TaskId, TaskState}};

pub enum TDLCommands {
    AddTask{
        task_title: String,
        response_sender: Sender<CommandResult>
    },
    ListTasks{
        response_sender: Sender<CommandResult>
    },
    ToggleTask{
        id: TaskId,
        state: TaskState,
        response_sender: Sender<CommandResult>
    },
    EditTitle{
        id: TaskId,
        task_title: String,
        response_sender: Sender<CommandResult>
    },
    RemoveTask{
        id: TaskId,
        response_sender: Sender<CommandResult>
    },
    CommandNone{
        response_sender: Sender<CommandResult>
    },
    Quit
}

pub enum CommandResult {
    TaskAdded,
    InvalidTitle,
    TasksList(Vec<(task::TaskId, task::TaskTitle, task::TaskState)>),
    EmptyTasksList,
    TaskToggled,
    InvalidIdOrState,
    TaskTitleEdited,
    InvalidId,
    TaskRemoved,
    NoCommand
}

pub fn match_command(command: &str, response_sender: Sender<CommandResult>) -> Result<TDLCommands, TDLErrors>{
    match command {
        "add" => {
            println!("Please name your new task: ");
            let task_title = title_validate(user_input()?)?;
            Ok(TDLCommands::AddTask { 
                task_title, 
                response_sender
            })
        },
        "list" => {
            Ok(TDLCommands::ListTasks {
                response_sender
            })
        },
        "toggle" => {
            println!("Please enter ID:");
            let id = check_id(user_input()?)?;
            println!("Please enter state:");
            let state = check_state(user_input()?)?;
            Ok(TDLCommands::ToggleTask {
                id, 
                state, 
                response_sender 
            })
        },
        "edit" => {
            println!("Please enter ID:");
            let id = check_id(user_input()?)?;
            println!("Please input new task title: ");
            let task_title = title_validate(user_input()?)?;
            Ok(TDLCommands::EditTitle { 
                id, 
                task_title, 
                response_sender 
            })
        },
        "delete" => {
            println!("Please enter ID:");
            let id = check_id(user_input()?)?;
            println!("Please confirm deletion");
            if input_confirmation(user_input()?)? {
                Ok(TDLCommands::RemoveTask { 
                    id, 
                    response_sender 
                })
            } else {
                Ok(TDLCommands::CommandNone{
                    response_sender
                })
            }
        },
        "help" => {
            println!(
                "add - add a new task\n\
                list - show all tasks\n\
                toggle - mark a task as done/undone\n\
                edit - edit a task title\n\
                delete - delete a task\n\
                quit - exit the program\n\
                help - show this help"
            );
            Ok(TDLCommands::CommandNone{
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