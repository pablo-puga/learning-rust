use crate::task::TaskId;

/// Main [Command] type for the crate.
/// It describes the chosen action to perform.
#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    List,
    Add(String),
    Do(TaskId),
    UnDo(TaskId),
    Delete(TaskId),
    Exit,
}

/// Any possible [Command] building error
#[derive(PartialEq, Eq, Debug)]
pub enum BuildError {
    MissingCommandName,
    MissingArgument(String),
    UnknownCommand,
    NotUsizeTaskId,
}

impl BuildError {
    pub fn val(&self) -> String {
        match &self {
            BuildError::MissingCommandName => String::from("Missing command name"),
            BuildError::MissingArgument(text) => {
                let mut val = String::from("Missing argument: ");
                val += text.as_str();

                val
            },
            BuildError::UnknownCommand => String::from("Unknown command"),
            BuildError::NotUsizeTaskId => String::from("Given task id is not an usize"),
        }
    }
}

fn parse_text_arg(input: Vec<String>) -> String {
    let mut arg = String::from("");
    let mut delimiter: Option<char> = None;
    let mut got_closing_delimiter = false;

    if input.len() == 1 {
        arg += input.get(0).unwrap();
        if (arg.starts_with('"') && arg.ends_with('"'))
            || (arg.starts_with('\'') && arg.ends_with('\''))
        {
            arg = String::from(arg.get(1..arg.len() - 1).unwrap());
        }
        return arg;
    }

    for piece in input {
        let piece = String::from(piece.trim());

        if delimiter.is_none() {
            if piece.starts_with('"') || piece.starts_with('\'') {
                delimiter = Some(piece.chars().next().unwrap());
                arg += piece.get(1..).unwrap();
                continue;
            } else {
                return piece;
            }
        }

        if !arg.is_empty() {
            if delimiter.is_none() {
                break;
            }

            arg += " ";
        }

        if delimiter.is_some() && piece.ends_with(delimiter.unwrap()) {
            arg += piece.get(..piece.len() - 1).unwrap();
            got_closing_delimiter = true;
            break;
        }

        arg += piece.as_str();
    }

    if !got_closing_delimiter {
        return String::from("");
    }

    arg
}

/// It builds a [Command] give the user input
///
/// # Example
///
/// ```rust
/// use todo_list::command::build_command;
///
/// let result = build_command("add 'This is a task test'");
/// match result {
///     Ok(command) => println!("Command: {:?}", command),
///     Err(error) => println!("Got error: {}", error.val()),
/// };
/// ```
pub fn build_command(input: &str) -> Result<Command, BuildError> {
    let input = input.trim();
    let mut input = input.split_whitespace().map(|i| i.to_owned());

    let command_name = match input.next() {
        Some(name) => name.to_lowercase(),
        None => return Err(BuildError::MissingCommandName),
    };

    match command_name.as_str() {
        "list" => Ok(Command::List),
        "add" => {
            let args: Vec<_> = input.collect();
            let parsed_arg = parse_text_arg(args);

            if parsed_arg.is_empty() {
                Err(BuildError::MissingArgument(String::from("add 'tast text'")))
            } else {
                Ok(Command::Add(parsed_arg))
            }
        },
        name @ ("do" | "undo" | "delete") => match input.next() {
            Some(str) => {
                let id = str.parse::<usize>();
                if id.is_err() {
                    Err(BuildError::NotUsizeTaskId)
                } else {
                    let command = match name {
                        "do" => Command::Do(TaskId::new(id.ok().unwrap())),
                        "undo" => Command::UnDo(TaskId::new(id.ok().unwrap())),
                        "delete" => Command::Delete(TaskId::new(id.ok().unwrap())),
                        _ => panic!("Should never be here"),
                    };
                    Ok(command)
                }
            },
            None => Err(BuildError::MissingArgument(format!("{} TASK_ID", name))),
        },
        "exit" => Ok(Command::Exit),
        _ => Err(BuildError::UnknownCommand),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_inputs() {
        let output = parse_text_arg(vec![String::from("input")]);
        assert_eq!(output, String::from("input"));

        let output = parse_text_arg(vec![String::from("'input'")]);
        assert_eq!(output, String::from("input"));

        let output = parse_text_arg(
            "input with more words that should be one"
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        );
        assert_eq!(output, String::from("input"));

        let output = parse_text_arg(
            "'input with more words'"
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        );
        assert_eq!(output, String::from("input with more words"));

        let output = parse_text_arg(
            "\"input with more words double quotes\""
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        );
        assert_eq!(output, String::from("input with more words double quotes"));

        let output = parse_text_arg(
            "\"not closed "
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        );
        assert_eq!(output, String::from(""));

        let output = parse_text_arg(
            "'input with more words after the quotes' these are more words 'other text'"
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        );
        assert_eq!(
            output,
            String::from("input with more words after the quotes")
        );
    }

    #[test]
    fn should_error_if_empty_input() {
        let result = build_command("");
        assert_eq!(result, Err(BuildError::MissingCommandName));
    }

    #[test]
    fn should_create_list_command() {
        let result = build_command("list  ");
        assert_eq!(result, Ok(Command::List));

        let result = build_command("this is not a list command");
        assert_ne!(result, Ok(Command::List));
    }

    #[test]
    fn should_create_add_command() {
        let task_text = "Test task";

        let result = build_command("add");
        assert!(result.is_err());
        assert!(result.err().unwrap().val().starts_with("Missing argument: add"));

        let result = build_command(format!(" add '{task_text}'").as_str());
        assert_eq!(result, Ok(Command::Add(String::from(task_text))));

        let result = build_command(format!(" add \"{task_text}\"").as_str());
        assert_eq!(result, Ok(Command::Add(String::from(task_text))));

        let result = build_command("this is not an add command");
        assert_ne!(result, Ok(Command::Add(String::from(task_text))));
    }

    #[test]
    fn should_create_do_command() {
        let result = build_command("do");
        assert!(result.is_err());
        assert!(result.err().unwrap().val().starts_with("Missing argument: do"));

        let result = build_command(" do 1");
        assert_eq!(result, Ok(Command::Do(TaskId::new(1))));

        let result = build_command(" do 1");
        assert_eq!(result, Ok(Command::Do(TaskId::new(1))));

        let result = build_command(" do not_a_number");
        assert_eq!(result, Err(BuildError::NotUsizeTaskId));

        let result = build_command("this is not an do command");
        assert_ne!(result, Ok(Command::Do(TaskId::new(1))));
    }

    #[test]
    fn should_create_undo_command() {
        let result = build_command("undo");
        assert!(result.is_err());
        assert!(result.err().unwrap().val().starts_with("Missing argument: undo"));

        let result = build_command(" undo 1");
        assert_eq!(result, Ok(Command::UnDo(TaskId::new(1))));

        let result = build_command(" undo 1");
        assert_eq!(result, Ok(Command::UnDo(TaskId::new(1))));

        let result = build_command(" undo not_a_number");
        assert_eq!(result, Err(BuildError::NotUsizeTaskId));

        let result = build_command("this is not an undo command");
        assert_ne!(result, Ok(Command::UnDo(TaskId::new(1))));
    }

    #[test]
    fn should_create_delete_command() {
        let result = build_command("delete");
        assert!(result.is_err());
        assert!(result
            .err()
            .unwrap()
            .val()
            .starts_with("Missing argument: delete"));

        let result = build_command(" delete 1");
        assert_eq!(result, Ok(Command::Delete(TaskId::new(1))));

        let result = build_command(" delete 1");
        assert_eq!(result, Ok(Command::Delete(TaskId::new(1))));

        let result = build_command(" delete not_a_number");
        assert_eq!(result, Err(BuildError::NotUsizeTaskId));

        let result = build_command("this is not a delete command");
        assert_ne!(result, Ok(Command::Delete(TaskId::new(1))));
    }

    #[test]
    fn should_create_exit_command() {
        let result = build_command("exit  ");
        assert_eq!(result, Ok(Command::Exit));

        let result = build_command("this is not an exit command");
        assert_ne!(result, Ok(Command::Exit));
    }

    #[test]
    fn should_error_if_unknown_command() {
        let result = build_command("an extrange command");
        assert_eq!(result, Err(BuildError::UnknownCommand));
    }
}
