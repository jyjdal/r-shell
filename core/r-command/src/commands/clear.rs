use crate::BaseCommand;

use r_common::action::CommandAction;
use r_common::args::CommandArg;
use r_context::context::Context;

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
