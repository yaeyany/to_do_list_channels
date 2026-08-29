use std::{process::exit, sync::mpsc::{Receiver, Sender, channel}, thread::{self}};

use crate::{commands::{CommandResult::{self, EmptyTasksList, InvalidId, InvalidIdOrState, InvalidTitle, TaskAdded, TaskTitleEdited, TaskToggled, TasksList}, TDLCommands::{self, AddTask, EditTitle, ListTasks, Quit, ToggleTask}, match_command}, errors::{TDLErrors, handle_error}, input::user_input, task::TaskCollection};

mod task;
mod errors;
mod input;
mod commands;

fn main() {
    println!("Welcome to the to do list. Please write \"add\" to add or \"quit\" to quit");
    let (sender, receiver) = channel::<TDLCommands>();
    let tasks = TaskCollection::new();
    let user = thread::spawn(move || {
        loop {
            let (response_sender, response_receiver) = channel::<CommandResult>();
            match process_command(response_sender) {
                Ok(command) => {
                    if sender.send(command).is_err() {
                        println!("Server unavailable");
                        break;
                    }
                    command_result(response_receiver.recv().unwrap());
                    continue;
                },
                Err(e) => {
                    handle_error(e);
                    continue;
                }
            }
        }
    });

    let server = thread::spawn(move || {
        server(tasks, receiver);
    });

    let _ = user.join();
    let _ = server.join();  
}

pub fn server(mut tasks: TaskCollection, receiver: Receiver<TDLCommands>) {
    loop {
        match receiver.recv() {
            Ok(AddTask {
                task_title, 
                response_sender 
            }) => {
                match task_title.try_into() {
                    Ok(task_title) => {
                        tasks.add(task_title).unwrap();
                        let _ = response_sender.send(CommandResult::TaskAdded);
                    },
                    Err(_) => {
                        let _ = response_sender.send(CommandResult::InvalidTitle);
                    },
                }
            },
            Ok(ListTasks {
                response_sender
            }) => {
                let tasks= tasks.list();
                if tasks.is_empty() {
                    let _ = response_sender.send(CommandResult::EmptyTasksList);
                } else {
                    let _ = response_sender.send(CommandResult::TasksList(tasks));
                }
            },
            Ok(ToggleTask { 
                id, 
                state, 
                response_sender 
            }) => {
                if tasks.toggle(id, state).is_err() {
                    let _ = response_sender.send(CommandResult::InvalidIdOrState);
                } else {
                    let _ = response_sender.send(CommandResult::TaskToggled);
                }
            },
            Ok(EditTitle {
                id,
                task_title,
                response_sender,
            }) => {
                match task_title.try_into() {
                    Ok(task_title) => {
                        if tasks.edit_title(id, task_title).is_err() {
                            let _ = response_sender.send(CommandResult::InvalidId);
                        } else {
                            let _ = response_sender.send(CommandResult::TaskTitleEdited);
                        };
                        
                    },
                    Err(_) => {
                        let _ = response_sender.send(CommandResult::InvalidTitle);
                    },
                }
            },
            Ok(Quit) => exit(0),
            Err(_) => break,
        }
    }
}

pub fn process_command(response_sender: Sender<CommandResult>) -> Result<TDLCommands, TDLErrors> {
    let input = user_input()?;
    let command = match_command(&input.to_lowercase(), response_sender)?;
    Ok(command)
}

pub fn command_result(command_result: CommandResult) {
    match command_result {
        TaskAdded => println!("Task successfully added"),
        InvalidTitle => println!("Invalid title"),
        TasksList(items) => {
            println!("Your list:");
            for (id, task, state) in items {
                println!("{id}: \"{task}\", {state}")
            }
        },
        EmptyTasksList => println!("Your list is empty"),
        TaskToggled => println!("Task successfully toggled"),
        InvalidIdOrState => println!("Please enter a valid ID and state"),
        TaskTitleEdited => println!("Task title edited"),
        InvalidId => println!("Task ID must be >=0 and within the list"),
    }
}

