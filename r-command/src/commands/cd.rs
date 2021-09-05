use crate::BaseCommand;
use r_context::CONTEXT;
use r_parser::command_args::{ArgValue, CommandArg};

pub struct Cd {}

impl BaseCommand for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self, args: Vec<CommandArg>) {
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

            // BUG 这里在切换盘符的时候会出问题，表现为目录显示不正确
            unsafe {
                let mut new_path = CONTEXT.current_dir.clone();
                new_path.push(path);
                if new_path.exists() {
                    CONTEXT.current_dir = new_path;
                } else {
                    println!("Path does not exist!")
                }
            }
        }
    }
}
