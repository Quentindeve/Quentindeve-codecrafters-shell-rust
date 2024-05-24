use crate::command::ErrorType;

pub fn builtins_exit(args: Vec<String>) -> Result<(), ErrorType> {
    let exit_code = args
        .get(1)
        .map(|code| code.parse::<i32>())
        .unwrap_or(Ok(0i32))
        .map_err(|_| ErrorType::ArgumentError(0))?;

    std::process::exit(exit_code);
}
