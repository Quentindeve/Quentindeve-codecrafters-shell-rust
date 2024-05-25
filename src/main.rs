#[allow(unused_imports)]
use std::io::{self, Write};

use command::BuiltinCommand;

pub mod builtins;
pub mod command;
pub mod utils;

fn main() {
    io::stdout().flush().unwrap();

    let builtins = command::get_builtins();

    loop {
        // Get input and split it into arguments
        let input = prompt_input();
        let args = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        // If nothing in buffer, we skip
        if args.len() == 0 {
            continue;
        }

        // We try to run builtin if it exists, if not we search in external commands
        let builtin_run = run_builtin_if_exists(builtins.clone(), args.clone());
        if !builtin_run {
            // If the command is not a builtin, we try to run it as an external command
            let external_path = utils::search_command_in_env(args.get(0).unwrap());

            match external_path {
                Some(path) => {
                    let _ = utils::execute_extern_command(path, args);
                }
                None => {
                    println!("{}: command not found", args.get(0).unwrap());
                }
            }
        }
    }
    // Wait for user input
}

fn prompt_input() -> String {
    // Print prompt
    print!("$ ");
    io::stdout().flush().unwrap();

    // Read stdin
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    input
}

fn run_builtin_if_exists(builtins: Vec<BuiltinCommand>, args: Vec<String>) -> bool {
    // Check if the command is a builtin and runs it if yes
    let command = builtins
        .iter()
        .find(|builtin| builtin.name == args.get(0).unwrap());

    match command {
        Some(builtin) => {
            let _ = builtin.execute(args).unwrap();
            return true;
        }
        None => {
            return false;
        }
    }
}
