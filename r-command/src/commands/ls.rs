use std::env;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::BaseCommand;

use r_parser::data::CommandArg;

pub struct Ls {}

impl BaseCommand for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn run(&self, _: Vec<CommandArg>) {
        let current_dir = &env::current_dir().unwrap();

        let path = Path::new(current_dir);

        let walker = WalkDir::new(path).max_depth(1).min_depth(1).into_iter();
        print!("  .\t..\t");
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            print!("{}\t", entry.unwrap().path().display());
        }
        println!("");
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    return entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false);
}
