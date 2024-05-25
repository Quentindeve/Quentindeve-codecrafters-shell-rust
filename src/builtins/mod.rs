pub mod echo;
pub mod exit;
pub mod r#type;

pub use crate::command::BuiltinCommand;

pub const BUILTIN_EXIT: BuiltinCommand = BuiltinCommand {
    name: "exit",
    callback: exit::builtins_exit,
};

pub const BUILTIN_ECHO: BuiltinCommand = BuiltinCommand {
    name: "echo",
    callback: echo::builtins_echo,
};

pub const BUILTIN_TYPE: BuiltinCommand = BuiltinCommand {
    name: "type",
    callback: r#type::builtins_type,
};
