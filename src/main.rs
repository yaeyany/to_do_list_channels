use std::{sync::mpsc::channel, thread::{self}};

use crate::task::TaskCollection;

mod task;
mod errors;
mod input;
mod commands;

pub struct SessionId(u32);

fn main() {
    let (sender, receiver) = channel::<SessionId>();
    let tasks = TaskCollection::new();
    let user = thread::spawn(move || {
        
    });

    let server = thread::spawn(move || {

    });
}
