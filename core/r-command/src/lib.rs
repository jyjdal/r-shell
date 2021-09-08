pub mod commands;

use commands::all_commands;

use r_parser::command_args::CommandArg;
use r_context::context::Context;
use r_context::action::CommandAction;

pub trait BaseCommand {
    fn name(&self) -> &str;

    fn run(&self, context: Context, input: &Vec<CommandArg>) -> Vec<CommandAction>;
}

use std::collections::HashMap;

pub fn init_commands() -> HashMap<String, Box<dyn BaseCommand>> {
    let mut res = HashMap::new();
    for command in all_commands() {
        res.insert(String::from(command.name()), command);
    }
    res
}
