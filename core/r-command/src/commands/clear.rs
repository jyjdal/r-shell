use crate::BaseCommand;

use r_common::{ShellAction, ShellError, CommandArg};
use r_context::context::Context;

pub struct Clear {}

impl BaseCommand for Clear {
    fn name(&self) -> &str {
        "clear"
    }

    fn run(&self, _: Context, _: &Vec<CommandArg>) -> Result<Vec<ShellAction>, ShellError> {
        let mut res = Vec::new();
        res.push(ShellAction::ClearHost);
        Ok(res)
    }
}
