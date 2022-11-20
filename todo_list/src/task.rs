use std::hash::Hash;

#[derive(Debug)]
pub enum TaskStatus {
    Pending,
    Done,
}

#[derive(Eq, Debug)]
pub struct TaskId(usize);

impl TaskId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn val(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct Task {
    id: TaskId,
    status: TaskStatus,
    text: String,
}

impl Task {
    pub fn new(id: TaskId, status: TaskStatus, text: &str) -> Self {
        Self {
            id,
            status,
            text: text.replace(';', ""),
        }
    }
}

impl Hash for TaskId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.0);
        state.finish();
    }
}

impl PartialEq for TaskId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
