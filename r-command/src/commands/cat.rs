use crate::BaseCommand;
use r_context::CONTEXT;
use r_parser::command_args::{ArgValue, CommandArg};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct Cat {}

impl BaseCommand for Cat {
    fn name(&self) -> &str {
        "cat"
    }

    fn run(&self, args: &Vec<CommandArg>) {
        if args.len() == 0 {
            println!("No file selected!");
            return;
        }

        let arg = args[0].clone();
        if let ArgValue::String(filename) = arg.value {
            unsafe {
                let mut path: PathBuf = CONTEXT.current_dir.clone();
                path.push(filename.clone());

                match File::open(path.clone()) {
                    Err(_) => {
                        println!("Unable to open file: {}", filename);
                    }
                    Ok(mut file) => {
                        let mut content = String::new();
                        file.read_to_string(&mut content).unwrap();
                        println!("{}", content);
                    }
                }
            }
        }
    }
}
