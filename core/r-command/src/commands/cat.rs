use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::BaseCommand;
use r_context::action::CommandAction;
use r_context::context::Context;
use r_parser::command_args::{ArgValue, CommandArg};

pub struct Cat {}

impl BaseCommand for Cat {
    fn name(&self) -> &str {
        "cat"
    }

    fn run(&self, context: Context, args: &Vec<CommandArg>) -> Vec<CommandAction> {
        if args.len() == 0 {
            println!("No file selected!");
            return vec![];
        }

        let arg = args[0].clone();
        if let ArgValue::String(filename) = arg.value {
            let mut path: PathBuf = context.current_dir;
            path.push(filename.clone());

            match File::open(path.clone()) {
                Err(_) => {
                    println!("Unable to open file: {}", filename);
                    return vec![];
                }
                Ok(mut file) => {
                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();
                    println!("{}", content);
                    return vec![];
                }
            }
        }
        vec![]
    }
}
