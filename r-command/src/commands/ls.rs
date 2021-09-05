use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::BaseCommand;

use r_context::CONTEXT;
use r_parser::command_args::CommandArg;

pub struct Ls {}

impl BaseCommand for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn run(&self, args: Vec<CommandArg>) {
        unsafe {
            let path = Path::new(&CONTEXT.current_dir);

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
                    print!("{}\t", entry.unwrap().path().display());
                }
            } else {
                for entry in walker.filter_entry(|e| !is_hidden(e)) {
                    print!("{}\t", entry.unwrap().path().display());
                }
            }
            println!("");
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
