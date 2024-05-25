use crate::{command::ErrorType, utils::search_command_in_env};

pub fn builtins_type(args: Vec<String>) -> Result<(), ErrorType> {
    if args.len() < 2 {
        eprintln!("Usage: type <command>");

        return Err(ErrorType::InsufficientArguments {
            min: 2,
            actual: args.len(),
        });
    }

    let command_name = args.get(1).unwrap();
    let builtins = crate::command::get_builtins();

    let builtin_match = builtins.iter().find(|builtin| builtin.name == command_name);

    // Found in builtins
    if let Some(_) = builtin_match {
        println!("{command_name} is a shell builtin");
        return Ok(());
    }

    // Found in $PATH
    if let Some(path) = search_command_in_env(command_name) {
        println!("{command_name} is {}", path.to_str().unwrap());
        return Ok(());
    }

    // Not found
    println!("{command_name} not found");

    Ok(())
}
