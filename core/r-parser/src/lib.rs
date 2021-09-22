use r_common::args::CommandAndArgs;

use regex::Regex;

pub fn parse_command(input: &mut String) -> CommandAndArgs {
    let re = Regex::new(" ").unwrap();
    let command_and_args = re.split(input).collect::<Vec<&str>>();

    let mut res: CommandAndArgs = CommandAndArgs {
        command: String::new(),
        args: Vec::new(),
    };

    for arg in command_and_args {
        // Never forget to trim the command.
        res.args.push(arg.trim().to_string());
    }
    res.command = res.args[0].clone();
    
    res
}
