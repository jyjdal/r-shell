use crate::commands::args::CdArgs;
use crate::BaseCommand;

use structopt::StructOpt;

use r_common::{CommandArgs, ShellAction, ShellError};
use r_context::context::Context;

pub struct Cd {}

impl BaseCommand for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self, context: Context, args: CommandArgs) -> Result<Vec<ShellAction>, ShellError> {
        match CdArgs::from_iter_safe(args) {
            Ok(args) => {
                let mut dest = context.current_dir.clone();
                dest.push(args.dest);
                match dest.exists() {
                    true => {
                        return Ok(vec![ShellAction::ChangePath(
                            dest.as_path().display().to_string(),
                        )]);
                    }
                    false => {
                        return Err(ShellError::PathNotExist);
                    }
                }
            }
            Err(e) => return Err(ShellError::ParseError(e.message)),
        }
    }
}
