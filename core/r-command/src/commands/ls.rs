use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::BaseCommand;

use r_context::context::Context;
use r_context::action::CommandAction;
use r_parser::command_args::CommandArg;

pub struct Ls {}

impl BaseCommand for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn run(&self, context: Context, args: &Vec<CommandArg>) -> Vec<CommandAction> {
        let path = Path::new(&context.current_dir);
        let walker = WalkDir::new(path).max_depth(1).min_depth(1).into_iter();

        let mut all: bool = false;
        for arg in args {
            if let Some(t) = arg.tag {
                if t == "a" {
                    all = true;
                }
            }
        }

        print!("  .\t..\t");
        if all {
            for entry in walker {
                display_path(entry.unwrap());
            }
        } else {
            for entry in walker.filter_entry(|e| !is_hidden(e)) {
                display_path(entry.unwrap());
            }
        }
        println!("");

        vec![]
    }
}

fn display_path(entry: DirEntry) {
    let path = entry.into_path();
    if let Some(n) = path.file_name() {
        if let Some(n) = n.to_str() {
            print!("{}", n);
            if path.is_dir() {
                #[cfg(windows)]
                {
                    print!("\\");
                }
                #[cfg(not(windows))]
                {
                    print!("/");
                }
            }
            print!("\t");
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    return entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false);
}
