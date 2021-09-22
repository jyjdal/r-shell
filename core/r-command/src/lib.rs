pub mod commands;

use commands::all_commands;

use r_common::{CommandArgs, ShellAction, ShellError};
use r_context::context::Context;

pub trait BaseCommand {
    fn name(&self) -> &str;

    fn run(&self, context: Context, input: CommandArgs) -> Result<Vec<ShellAction>, ShellError>;
}

use std::collections::HashMap;

pub fn init_commands() -> HashMap<String, Box<dyn BaseCommand>> {
    let mut res = HashMap::new();
    for command in all_commands() {
        res.insert(String::from(command.name()), command);
    }
    res
}
