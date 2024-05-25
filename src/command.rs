pub type CommandCallback = fn(Vec<String>) -> Result<(), ErrorType>;

use crate::builtins::*;

#[derive(Debug)]
pub enum ErrorType {
    CommandNotFound(String),
    ArgumentError(usize),
    NotNullExitCode(i32),
    InsufficientArguments { min: usize, actual: usize },
}

pub struct BuiltinCommand<'a> {
    pub name: &'a str,
    pub callback: CommandCallback,
}

impl<'a> BuiltinCommand<'a> {
    pub fn execute(&self, args: Vec<String>) -> Result<(), ErrorType> {
        (self.callback)(args)
    }
}

pub fn get_builtins() -> Vec<BuiltinCommand<'static>> {
    vec![BUILTIN_EXIT, BUILTIN_ECHO, BUILTIN_TYPE]
}
