pub mod echo;
pub mod exit;

pub use crate::command::BuiltinCommand;

pub const BUILTIN_EXIT: BuiltinCommand = BuiltinCommand {
    name: "exit",
    callback: exit::builtins_exit,
};

pub const BUILTIN_ECHO: BuiltinCommand = BuiltinCommand {
    name: "echo",
    callback: echo::builtins_echo,
};
