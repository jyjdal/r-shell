pub mod data;
use data::{CommandAndArgs, CommandArg, ArgValue};

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
            res.args.push(CommandArg {
                tag: Some(&arg.trim()[1..]),
                value: ArgValue::Bool(true),
            })
        }
    }
    res
}
