use std::process::exit;

use crate::BaseCommand;

use r_parser::data::CommandArg;

pub struct Exit {}

impl BaseCommand for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, _: Vec<CommandArg>) {
        exit(0);
    }
}
