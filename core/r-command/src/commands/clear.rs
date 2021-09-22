use crate::BaseCommand;

use r_common::{CommandArgs, ShellAction, ShellError};
use r_context::context::Context;

pub struct Clear {}

impl BaseCommand for Clear {
    fn name(&self) -> &str {
        "clear"
    }

    fn run(&self, _: Context, _: CommandArgs) -> Result<Vec<ShellAction>, ShellError> {
        let mut res = Vec::new();
        res.push(ShellAction::ClearHost);
        Ok(res)
    }
}
