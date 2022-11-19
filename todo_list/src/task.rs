pub enum TaskStatus {
    Pending,
    Done,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TaskId(usize);

impl TaskId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn val(&self) -> usize {
        self.0
    }
}

pub struct Task {
    id: TaskId,
    status: TaskStatus,
    text: String,
}
