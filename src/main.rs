use std::{process::exit, sync::mpsc::{Receiver, channel}, thread::{self}};

use crate::{commands::{TDLCommands::{self, AddTask, Quit}, match_command}, errors::{TDLErrors, handle_error}, input::user_input, task::TaskCollection};

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
             match process_command() {
                Ok(command) => {
                    sender.send(command).unwrap();
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

    user.join();
    server.join();  
}

pub fn server(mut tasks: TaskCollection, receiver: Receiver<TDLCommands>) {
    loop {
        match receiver.recv().unwrap() {
            AddTask(task_title) => {tasks.add(task_title).unwrap();},
            Quit => exit(0),
        }
    }
}

pub fn process_command() -> Result<TDLCommands, TDLErrors> {
    let input = user_input()?;
    let command = match_command(&input.to_lowercase())?;
    Ok(command)
}

