pub mod fun;
pub mod general;
pub mod moderation;

use rusty::{Command, Context, Error};

pub fn commands() -> Vec<Command> {
    fun::commands()
        .into_iter()
        .chain(moderation::commands())
        .chain(general::commands())
        .collect()
}
