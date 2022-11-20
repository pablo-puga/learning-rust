use std::fmt;
use std::hash::Hash;

#[derive(Debug)]
pub enum TaskStatus {
    Pending,
    Done,
}

impl TaskStatus {
    pub fn val(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Done => String::from("done"),
        }
    }
}

#[derive(Eq, Debug, Clone, Copy)]
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
    pub fn from_parts(id: TaskId, status: TaskStatus, text: &str) -> Self {
        Self {
            id,
            status,
            text: text.replace(';', ""),
        }
    }

    pub fn new(id: TaskId, text: &str) -> Self {
        Self {
            id,
            status: TaskStatus::Pending,
            text: text.replace(';', ""),
        }
    }

    pub fn r#do(&mut self) {
        self.status = TaskStatus::Done;
    }

    pub fn undo(&mut self) {
        self.status = TaskStatus::Pending;
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn to_csv(&self) -> String {
        format!("{};{};{}", self.id.val(), self.status.val(), self.text)
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

impl PartialOrd for TaskId {
    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.gt(other) {
            Some(std::cmp::Ordering::Greater)
        } else if self.lt(other) {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Ord for TaskId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)       
    }
}