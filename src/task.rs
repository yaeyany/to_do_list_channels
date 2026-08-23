use std::collections::BTreeMap;
use std::ops::AddAssign;

use crate::input::{title_validate, user_input};
use crate::errors::TDLErrors;
use crate::task::TaskState::ToDo;

//Custom task structs
pub struct Task {
    title: TaskTitle,
    id: TaskId,
    state: TaskState,
}

//Task components
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TaskId(u32);
pub struct TaskTitle(String);
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

impl AddAssign<u32> for TaskId {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

//Methods
impl TaskCollection {
    pub fn new() -> Self {
        TaskCollection {tasks: BTreeMap::new(), counter: TaskId(0)}
    }

    pub fn add(&mut self) -> Result<TaskId, TDLErrors>{
        let valid_title: TaskTitle = user_input()?.try_into()?;
        let id = self.counter;
        let task = Task {
            title: valid_title,
            id: id,
            state: ToDo,
        };
        self.tasks.insert(self.counter, task);
        self.counter += 1;
        Ok(id)
    }
}