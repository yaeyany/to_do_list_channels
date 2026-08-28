use std::{process::exit, sync::mpsc::{Receiver, Sender, channel}, thread::{self}};

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
            let (response_sender, response_receiver) = channel::<String>();
            match process_command(response_sender) {
                Ok(command) => {
                    if sender.send(command).is_err() {
                        println!("Server unavailable");
                    }
                    println!("{}", response_receiver.recv().unwrap());
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
        match receiver.recv() {
            Ok(AddTask {
                task_title, 
                response_sender 
            }) => {
                match task_title.try_into() {
                    Ok(task_title) => {
                        tasks.add(task_title).unwrap();
                        let _ = response_sender.send("Success".to_string());
                    },
                    Err(_) => {
                        let _ = response_sender.send("Invalid title".to_string());
                    },
                }
            },
            Ok(Quit) => exit(0),
            Err(_) => break,
        }
    }
}

pub fn process_command(response_sender: Sender<String>) -> Result<TDLCommands, TDLErrors> {
    let input = user_input()?;
    let command = match_command(&input.to_lowercase(), response_sender)?;
    Ok(command)
}

