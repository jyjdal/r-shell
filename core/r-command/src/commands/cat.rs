use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::BaseCommand;
use r_common::{ArgValue, CommandArg, ShellAction, ShellError};
use r_context::context::Context;

pub struct Cat {}

impl BaseCommand for Cat {
    fn name(&self) -> &str {
        "cat"
    }

    fn run(
        &self,
        context: Context,
        args: &Vec<CommandArg>,
    ) -> Result<Vec<ShellAction>, ShellError> {
        if args.len() == 0 {
            return Err(ShellError::FileNotSpecified);
        }

        let arg = args[0].clone();
        if let ArgValue::String(filename) = arg.value {
            let mut path: PathBuf = context.current_dir;
            path.push(filename.clone());

            match File::open(path.clone()) {
                Err(_) => {
                    return Err(ShellError::CannotOpenFile(filename));
                }
                Ok(mut file) => {
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();
                    println!("{}", content);
                    return Ok(vec![]);
                }
            }
        }
        Ok(vec![])
    }
}
