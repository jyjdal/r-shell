use r_common::args::{ArgValue, CommandAndArgs, CommandArg};

use regex::Regex;

// TODO 这里优化命令解析流程，可能需要编译原理的知识[doge]
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
            res.args
                .push(CommandArg::new_bool_value(&arg[1..].trim(), true));
        } else {
            res.args.push(CommandArg {
                tag: None,
                value: ArgValue::String(arg.trim().to_string()),
            })
        }
    }
    res
}
