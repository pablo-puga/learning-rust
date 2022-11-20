use std::collections::HashMap;
use std::fs;
use std::io;
use std::path;

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

        while let Some(line) = content.lines().next() {
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
            let task = Task::new(TaskId::new(id), status, text);
            tasks.insert(TaskId::new(id), task);
        }

        Ok(Self {
            file,
            printer,
            tasks,
        })
    }
}
