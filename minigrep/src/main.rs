use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //The env::args() function works with unicode data if we need to use invalid unicode use env::args_os()
    //The collect() method turns an iterator into a collection
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // prints to standard error stream
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
