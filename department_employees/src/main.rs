use std::collections::HashMap;
use std::io;
use std::io::Write;

use regex::Regex;

fn ask_for_action() -> String {
    print!("Enter action: ");
    let _ = io::stdout().flush();

    let mut action = String::new();
    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");
    
    return String::from(action.trim());
}

fn main() {
    let add_action_regexp = Regex::new(r"^Add (?P<employee>\w+) to (?P<department>\w+)$").unwrap();
    let show_action_regexp = Regex::new(r"^Show (?P<department>\w+)$").unwrap();
    let exit_action_regexp = Regex::new(r"^Exit").unwrap();

    let mut department_employees: HashMap<String, Vec<String>> = HashMap::new();

    'main: loop {
        let action = ask_for_action();

        if add_action_regexp.is_match(&action) {
            let action_matches = add_action_regexp.captures(&action).unwrap();    
            let employee = &action_matches["employee"];
            let department = &action_matches["department"];

            let employees = department_employees.entry(String::from(department)).or_insert(Vec::new());
            employees.push(String::from(employee));
            
            println!("Employee {} added to the {} deparment", employee, department);
        } else if show_action_regexp.is_match(&action) {
            let action_matches = show_action_regexp.captures(&action).unwrap();
            let department = &action_matches["department"];
            
            if department_employees.contains_key(&String::from(department)) {
                let employees = department_employees.get(&String::from(department)).unwrap();
                println!("The employees of the {} department are: {}", department, employees.join(", "));
            } else {
                println!("The department {} does not exists!", department);
            }
        } else if exit_action_regexp.is_match(&action) {
            break 'main;
        } else {
            println!("Unknwon action!");
        }
    }
}
