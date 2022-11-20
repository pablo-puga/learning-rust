use std::io;
use std::io::Write;

use todo_list::command::Command;
use todo_list::command::build_command;
use todo_list::printer::Printer;

const TASK_FILE: &str = "tasks.csv";

fn main() {
    let printer = Printer::new();

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
