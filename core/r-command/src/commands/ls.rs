use std::path::Path;
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

use crate::commands::args::LsArgs;
use crate::BaseCommand;

use r_common::{CommandArgs, ShellAction, ShellError};
// use r_common::args::CommandArg;
use r_context::context::Context;

pub struct Ls {}

impl BaseCommand for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn run(&self, context: Context, args: CommandArgs) -> Result<Vec<ShellAction>, ShellError> {
        let path = Path::new(&context.current_dir);
        let walker = WalkDir::new(path).max_depth(1).min_depth(1).into_iter();

        match LsArgs::from_iter_safe(args) {
            Ok(args) => {
                print!("  .\t..\t");
                if args.all {
                    for entry in walker {
                        display_path(entry.unwrap());
                    }
                } else {
                    for entry in walker.filter_entry(|e| !is_hidden(e)) {
                        display_path(entry.unwrap());
                    }
                }
                println!("");
                Ok(vec![])
            }
            Err(e) => Err(ShellError::ParseError(e.message)),
        }
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
