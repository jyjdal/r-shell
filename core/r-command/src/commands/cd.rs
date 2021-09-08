use crate::BaseCommand;

use r_context::action::CommandAction;
use r_context::context::Context;
use r_parser::command_args::{ArgValue, CommandArg};

pub struct Cd {}

impl BaseCommand for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self, context: Context, args: &Vec<CommandArg>) -> Vec<CommandAction> {
        if args.len() == 0 {
            return Vec::new();
        }
        let arg = args[0].clone();
        if let ArgValue::String(path) = arg.value {
            if path == String::from(".") {
                return Vec::new();
            } else if path == String::from("..") {
                let mut new_path = context.current_dir;
                new_path.pop();
                return vec![CommandAction::ChangePath(format!("{}", new_path.display()))];
            }

            let mut new_path = context.current_dir;
            new_path.push(path);
            if new_path.exists() && new_path.is_dir() {
                return vec![CommandAction::ChangePath(format!("{}", new_path.display()))];
            } else {
                println!("Path does not exist!")
            }
        }
        vec![]
    }
}
