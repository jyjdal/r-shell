use std::path::Path;
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

use crate::commands::args::LsArgs;
use crate::BaseCommand;

use r_common::{CommandArgs, ShellAction, ShellError};
use r_context::context::Context;

pub struct Ls {}

impl BaseCommand for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn run(&self, context: Context, args: CommandArgs) -> Result<Vec<ShellAction>, ShellError> {
        let path = Path::new(&context.current_dir);
        let walker = WalkDir::new(path).max_depth(1).min_depth(1).into_iter();

        let mut entries: Vec<DirEntry> = vec![];

        match LsArgs::from_iter_safe(args) {
            Err(e) => Err(ShellError::ParseError(e.message)),
            Ok(args) => {
                if !args.all {
                    for entry in walker.filter_entry(|e| !is_hidden(e)) {
                        entries.push(entry.unwrap());
                    }
                } else {
                    for entry in walker {
                        entries.push(entry.unwrap());
                    }
                }
                if args.reverse {
                    entries.reverse();
                }

                // The actual output period.
                print!("  .\t..\t");
                for entry in entries {
                    display_path(entry);
                }
                println!("");
                Ok(vec![])
            }
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
