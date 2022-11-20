use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path;

use crate::command::Command;
use crate::printer::Printer;
use crate::task::Task;
use crate::task::TaskId;
use crate::task::TaskStatus;

#[derive(Debug)]
pub struct TaskList {
    file: String,
    printer: Box<Printer>,
    tasks: HashMap<TaskId, Task>,
}

impl TaskList {
    pub fn new(printer: Box<Printer>, file: &str) -> Result<Self, io::Error> {
        let file = String::from(file);
        let mut tasks = HashMap::new();

        if !path::Path::new(&file).exists() {
            return Ok(Self {
                file,
                printer,
                tasks,
            });
        }

        let content = fs::read_to_string(&file)?;
        let lines = content.lines();

        for line in lines {
            let pieces: Vec<_> = line.split(';').collect();

            if pieces.len() != 3 {
                printer
                    .warning(format!("Ignoring task '{}' due to missmatched parts", line).as_str());
                continue;
            }

            let id: usize = match pieces[0].parse() {
                Ok(i) => i,
                Err(_) => {
                    printer.warning(format!("Ignoring task '{}' due to invalid id", line).as_str());
                    continue;
                },
            };

            let status = match pieces[1] {
                "pending" => TaskStatus::Pending,
                "done" => TaskStatus::Done,
                _ => {
                    printer.warning(
                        format!("Ignoring task '{}' due to invalid status", line).as_str(),
                    );
                    continue;
                },
            };

            let text = pieces[2];
            let id = TaskId::new(id);
            let task = Task::from_parts(id, status, text);
            tasks.insert(id, task);
        }

        Ok(Self {
            file,
            printer,
            tasks,
        })
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::Add(text) => self.add_task(&text),
            Command::Delete(id) => self.delete_task(id),
            Command::Do(id) => self.do_task(id),
            Command::List => self.print_tasks(),
            Command::UnDo(id) => self.undo_task(id),
            _ => (),
        }
    }

    fn do_task(&mut self, id: TaskId) {
        let task = self.tasks.get_mut(&id);
        if task.is_none() {
            let msg = format!("Unknown task with key {}", id);
            self.printer.warning(&msg);
            return;
        }

        let task = task.unwrap();
        task.r#do();

        self.sync_to_file();
    }

    fn undo_task(&mut self, id: TaskId) {
        let task = self.tasks.get_mut(&id);
        if task.is_none() {
            let msg = format!("Unknown task with key {}", id);
            self.printer.warning(&msg);
            return;
        }

        let task = task.unwrap();
        task.undo();

        self.sync_to_file();
    }

    fn print_tasks(&self) {
        let mut ids: Vec<_> = self.tasks.keys().collect();
        ids.sort();

        for id in ids {
            let task = self.tasks.get(id).unwrap();
            let str = format!("{}\t{}\t\t{}", task.id(), task.status().val(), task.text());
            self.printer.notice(&str);
        }
    }

    fn add_task(&mut self, text: &str) {
        let id = self.get_next_task_id();
        let task = Task::new(id, text);
        self.tasks.insert(id, task);

        let msg = format!("Task successfully created with id {}", id);
        self.printer.notice(&msg);

        self.sync_to_file();
    }

    fn get_next_task_id(&mut self) -> TaskId {
        let mut ids: Vec<_> = self.tasks.keys().collect();
        if ids.is_empty() {
            return TaskId::new(1);
        }

        ids.sort();
        for index in 0..ids[ids.len() - 1].val() {
            let key = ids[index].val();
            let next_key = match ids.get(index + 1) {
                Some(k) => k.val(),
                None => break,
            };

            if key != next_key - 1 {
                return TaskId::new(key + 1);
            }
        }

        TaskId::new(ids.last().unwrap().val() + 1)
    }

    fn delete_task(&mut self, id: TaskId) {
        match self.tasks.remove(&id) {
            Some(_) => {
                let msg = format!("Task with key {} successfully deleted", id);
                self.printer.notice(&msg);
            },
            None => {
                let msg = format!("Unknown task with key {}", id);
                self.printer.warning(&msg);
            },
        }

        self.sync_to_file();
    }

    fn sync_to_file(&self) {
        let mut file = match File::create(&self.file) {
            Ok(f) => f,
            Err(e) => {
                let msg = format!("Error while opening file to sync '{}'", e);
                self.printer.error(&msg);
                return;
            }
        };

        let mut ids: Vec<_> = self.tasks.keys().collect();
        ids.sort();

        for id in ids {
            let task = self.tasks.get(id).unwrap();
            let str = format!("{}\n", task.to_csv());
            if let Err(e) = file.write(str.as_bytes()) {
                let msg = format!("Error while printing task '{}' to file '{}'", task.text(), e);
                self.printer.error(&msg);
                return;
            }
        }

        if let Err(e) = file.flush() {
            let msg = format!("Error while flushing buffer '{}'", e);
            self.printer.error(&msg);
        }
    }
}
