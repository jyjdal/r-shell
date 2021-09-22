use std::fs::File;
use std::io::Read;

use crate::commands::args::CatArgs;
use crate::BaseCommand;
use structopt::StructOpt;

use r_common::{CommandArgs, ShellAction, ShellError};
use r_context::context::Context;

pub struct Cat {}

impl BaseCommand for Cat {
    fn name(&self) -> &str {
        "cat"
    }

    fn run(&self, context: Context, args: CommandArgs) -> Result<Vec<ShellAction>, ShellError> {
        match CatArgs::from_iter_safe(args) {
            Ok(args) => {
                let mut file_path = context.current_dir.clone();
                file_path.push(args.file_name.clone());
                let mut file = match File::open(file_path) {
                    Err(_) => {
                        return Err(ShellError::CannotOpenFile(args.file_name));
                    }
                    Ok(file) => file,
                };
                let mut res = String::new();
                match file.read_to_string(&mut res) {
                    Ok(_) => return Ok(vec![ShellAction::OutputResult(res)]),
                    Err(_) => return Err(ShellError::CannotReadFile(args.file_name)),
                }
            }
            Err(e) => Err(ShellError::ParseError(e.message)),
        }
    }
}
