#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    io::stdout().flush().unwrap();

    let commands: Vec<String> = vec![];
    loop {
        let input = prompt_input();
        if !commands.contains(&input) {
            println!("{}: command not found", &input.trim());
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
