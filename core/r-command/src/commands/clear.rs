use crate::BaseCommand;

use r_context::action::CommandAction;
use r_context::context::Context;
use r_parser::command_args::CommandArg;

pub struct Clear {}

impl BaseCommand for Clear {
    fn name(&self) -> &str {
        "clear"
    }

    fn run(&self, _: Context, _: &Vec<CommandArg>) -> Vec<CommandAction> {
        let mut res = Vec::new();
        res.push(CommandAction::ClearHost);
        res
    }
}
