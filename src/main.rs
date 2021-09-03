// 为了支持flush方法
use std::io;
use std::io::Write;

use r_command::init_commands;
use r_parser::parse_command;

fn main() {
    let command_map = init_commands();

    loop {
        print!("r-shell> ");
        io::stdout().flush().unwrap();
        // 获取用户输入的命令
        let mut buf: String = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line.");

        // // 判空，如果为空自动进入下一轮循环
        if buf.len() == 0 {
            std::process::exit(0);
        }

        // // 匹配命令，执行命令
        let res = parse_command(&mut buf);

        if let Some(t) = command_map.get(res.command) {
            t.run(res.args.clone());
        } else {
            let args = res.clone().to_vec();
            if let Err(_) = std::process::Command::new(res.command).args(args).status() {
                println!("Unknown command!");
            }
        }
    }
}
