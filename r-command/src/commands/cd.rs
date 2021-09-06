use crate::BaseCommand;
use r_context::CONTEXT;
use r_parser::command_args::{ArgValue, CommandArg};

pub struct Cd {}

impl BaseCommand for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self, args: &Vec<CommandArg>) {
        if args.len() == 0 {
            return;
        }
        let arg = args[0].clone();
        if let ArgValue::String(path) = arg.value {
            if path == String::from(".") {
                return;
            } else if path == String::from("..") {
                unsafe {
                    CONTEXT.current_dir.pop();
                }
                return;
            }

            unsafe {
                let mut new_path = CONTEXT.current_dir.clone();
                new_path.push(path);
                if new_path.exists() && new_path.is_dir() {
                    CONTEXT.current_dir = new_path;
                } else {
                    println!("Path does not exist!")
                }
            }
        }
    }
}
