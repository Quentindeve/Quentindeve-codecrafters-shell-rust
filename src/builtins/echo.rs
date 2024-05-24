use crate::command::ErrorType;

pub fn builtins_echo(args: Vec<String>) -> Result<(), ErrorType> {
    let text = args
        .iter()
        .skip(1)
        .fold(String::new(), |acc, arg| acc + arg + " ");

    print!("{}", text);

    Ok(())
}
