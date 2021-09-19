use crate::BaseCommand;

use r_common::{ArgValue, CommandArg, ShellAction, ShellError};
use r_context::context::Context;

pub struct Cd {}

impl BaseCommand for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(
        &self,
        context: Context,
        args: &Vec<CommandArg>,
    ) -> Result<Vec<ShellAction>, ShellError> {
        if args.len() == 0 {
            return Err(ShellError::PathNotSpecified);
        }
        let arg = args[0].clone();
        if let ArgValue::String(path) = arg.value {
            if path == String::from(".") {
                return Ok(vec![]);
            } else if path == String::from("..") {
                let mut new_path = context.current_dir;
                new_path.pop();
                return Ok(vec![ShellAction::ChangePath(format!(
                    "{}",
                    new_path.display()
                ))]);
            }

            let mut new_path = context.current_dir;
            new_path.push(path);
            if new_path.exists() && new_path.is_dir() {
                return Ok(vec![ShellAction::ChangePath(format!(
                    "{}",
                    new_path.display()
                ))]);
            } else {
                return Err(ShellError::PathNotExist);
            }
        }
        Ok(vec![])
    }
}
