use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, AddAssign};

use crate::input::title_validate;
use crate::errors::TDLErrors::{self, InvalidTaskIdLookup};
use crate::task::TaskState::ToDo;

//Custom task structs
pub struct Task {
    title: TaskTitle,
    state: TaskState,
}

//Task components
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TaskId(u32);

#[derive(Clone)]
pub struct TaskTitle(String);

#[derive(Clone)]
pub enum TaskState {
    ToDo,
    InProgress,
    Done
}

//Task collection
pub struct TaskCollection {tasks: BTreeMap<TaskId, Task>, counter: TaskId}

//Traits
impl TryFrom<String> for TaskTitle {
    type Error = TDLErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = title_validate(value)?;
        Ok(Self(valid))
    }
}

impl Display for TaskTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u32> for TaskId {
    type Error = TDLErrors;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(TaskId(value))
    }
}

impl AddAssign<u32> for TaskId {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add<u32> for TaskId {
    type Output = TaskId;

    fn add(self, rhs: u32) -> TaskId {
        TaskId(self.0 + rhs)
    }
}

impl Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToDo => write!(f, "To do"),
            TaskState::InProgress => write!(f, "In progress"),
            TaskState::Done => write!(f, "Done"),
        }
    }
}

//Methods
impl TaskCollection {
    pub fn new() -> Self {
        TaskCollection {tasks: BTreeMap::new(), counter: TaskId(0)}
    }

    pub fn add(&mut self, title: TaskTitle) -> Result<TaskId, TDLErrors>{
        let id = self.counter;
        let task = Task {
            title: title,
            state: ToDo,
        };
        self.tasks.insert(self.counter, task);
        self.counter += 1;
        Ok(id)
    }

    pub fn list(&self) -> Vec<(TaskId, TaskTitle, TaskState)> {
        self.tasks.iter().map(|(id, task)| (id.clone(), task.title.clone(), task.state.clone())).collect()
    }

    pub fn toggle(&mut self, id: TaskId, state: TaskState) -> Result<(), TDLErrors>{
        if let Some(task) = self.tasks.get_mut(&id) {
            task.state = state;
            Ok(())
        } else {
            Err(InvalidTaskIdLookup)
        }
    }

    pub fn edit_title(&mut self, id: TaskId, title: String) -> Result<(), TDLErrors>{
        if let Some(task) = self.tasks.get_mut(&id) {
            task.title = title.try_into()?;
            Ok(())
        } else {
            Err(InvalidTaskIdLookup)
        }
    }

}