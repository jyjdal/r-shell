use crate::BaseCommand;

use r_common::action::CommandAction;
use r_common::args::CommandArg;
use r_context::context::Context;

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
