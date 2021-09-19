use crate::BaseCommand;

use r_common::{CommandArg, ShellAction, ShellError};
use r_context::context::Context;

pub struct Exit {}

impl BaseCommand for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, _: Context, _: &Vec<CommandArg>) -> Result<Vec<ShellAction>, ShellError> {
        let mut res = Vec::new();
        res.push(ShellAction::Exit(0));
        Ok(res)
    }
}
