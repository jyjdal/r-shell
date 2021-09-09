extern crate crossterm;
use crossterm::Command as CCommand;

use std::collections::HashMap;
use std::env::current_dir;
use std::fs::canonicalize;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use r_command::{init_commands, BaseCommand};
use r_common::action::CommandAction;
use r_context::context::Context;
use r_parser::parse_command;

pub struct App {
    pub context: Context,
    pub command_map: HashMap<String, Box<dyn BaseCommand>>,
}

impl App {
    pub fn run() {
        let mut app = Self {
            context: Context {
                current_dir: current_dir().unwrap(),
            },
            command_map: init_commands(),
        };
        app.main()
    }

    pub fn main(&mut self) {
        loop {
            // 输出提示信息
            print!("r-shell {}> ", self.context.current_dir.display());
            stdout().flush().unwrap();
            // 获取用户输入
            let mut buf: String = String::new();
            stdin().read_line(&mut buf).expect("Failed to read line.");
            // 判空，如果为空自动进入下一轮循环
            if buf.len() == 0 {
                std::process::exit(0);
            }

            // 匹配命令，执行命令
            let res = parse_command(&mut buf);

            if let Some(t) = self.command_map.get(res.command) {
                let actions = t.run(self.context.clone(), &res.args);
                for action in actions {
                    match action {
                        CommandAction::ClearHost => {
                            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
                                .execute_winapi()
                                .unwrap();
                        }
                        CommandAction::Exit(code) => {
                            std::process::exit(code);
                        }
                        CommandAction::ChangePath(new_path) => {
                            // BUG 显示绝对路径时会有 \\?\ 字符串出现在开始
                            // 但是好在bug不在我这里[doge]
                            // https://github.com/rust-lang/rust/issues/42869
                            self.context.current_dir =
                                canonicalize(PathBuf::from(new_path)).unwrap();
                        }
                    }
                }
            } else {
                let args = res.clone().to_vec();
                if let Err(_) = std::process::Command::new(res.command).args(args).status() {
                    println!("Unknown command!");
                }
            }
        }
    }
}
