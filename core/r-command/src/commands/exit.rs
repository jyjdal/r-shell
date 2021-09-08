use crate::BaseCommand;

use r_context::action::CommandAction;
use r_context::context::Context;
use r_parser::command_args::CommandArg;

pub struct Exit {}

impl BaseCommand for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, _: Context, _: &Vec<CommandArg>) -> Vec<CommandAction> {
        let mut res = Vec::new();
        res.push(CommandAction::Exit(0));
        res
    }
}
