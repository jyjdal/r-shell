pub mod data;
use data::{ArgValue, CommandAndArgs, CommandArg};

use regex::Regex;

pub fn parse_command(input: &mut String) -> CommandAndArgs {
    let re = Regex::new(" ").unwrap();
    let command_group: Vec<&str> = re.split(input).collect();

    let mut iter = command_group.iter();
    let command = iter.next().unwrap().trim();
    let mut res: CommandAndArgs = CommandAndArgs {
        command: command,
        args: Vec::<CommandArg>::new(),
    };

    for arg in iter {
        if arg.starts_with("-") {
            res.args.push(CommandArg::new_bool_value(&arg[1..], true));
        } else {
            res.args.push(CommandArg {
                tag: None,
                value: ArgValue::String(arg.to_string()),
            })
        }
    }
    res
}
