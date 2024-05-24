pub mod exit;

pub use crate::command::BuiltinCommand;

pub const BUILTIN_EXIT: BuiltinCommand = BuiltinCommand {
    name: "exit",
    callback: exit::builtins_exit,
};
