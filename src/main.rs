#[allow(unused_imports)]
use std::io::{self, Write};

pub mod builtins;
pub mod command;

fn main() {
    io::stdout().flush().unwrap();

    let builtins = command::get_builtins();

    loop {
        let input = prompt_input();
        let args = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if args.len() == 0 {
            continue;
        }

        let command = builtins
            .iter()
            .find(|builtin| builtin.name == args.get(0).unwrap());

        match command {
            Some(builtin) => {
                let result = builtin.execute(args);
            }
            None => {
                println!("{}: command not found", &input.trim());
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
