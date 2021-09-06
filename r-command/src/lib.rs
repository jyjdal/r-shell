pub mod commands;

use commands::all_commands;

use r_parser::command_args::CommandArg;

pub trait BaseCommand {
    fn name(&self) -> &str;

    fn run(&self, input: &Vec<CommandArg>);
}

use std::collections::HashMap;

pub fn init_commands() -> HashMap<String, Box<dyn BaseCommand>> {
    let mut res = HashMap::new();
    for command in all_commands() {
        res.insert(String::from(command.name()), command);
    }
    res
}
