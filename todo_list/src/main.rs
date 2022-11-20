use std::io;
use std::io::Write;
use std::process;

use todo_list::command::Command;
use todo_list::command::build_command;
use todo_list::printer::Printer;
use todo_list::task_list::TaskList;

const TASK_FILE: &str = "tasks.csv";

fn main() {
    let printer = Box::new(Printer::new());
    let task_list = match TaskList::new(Box::clone(&printer), TASK_FILE) {
        Ok(tl) => tl,
        Err(e) => {
            let msg = format!("Unable to create Task List due to previous error: {}", e);
            printer.error(&msg);
            process::exit(1);
        }
    };

    dbg!(task_list);

    printer.notice("Welcome to the task manager!");

    'main: loop {
        let input = match ask_user_input() {
            Ok(i) => i,
            Err(e) => {
                printer.error(format!("Unable to read input: {:?}", e).as_str());
                continue;
            }
        };

        let command = match build_command(input.as_str()) {
            Ok(c) => c,
            Err(e) => {
                printer.error(e.val().as_str());
                continue;
            }
        };

        if command == Command::Exit {
            printer.notice("Exiting...");
            break 'main;
        }

        dbg!(command);
    }

    printer.notice("Good bye!");
}

fn ask_user_input() -> Result<String, io::Error> {
    print!("CLI > ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    return Ok(String::from(input.trim()));
}
