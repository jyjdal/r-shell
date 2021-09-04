use std::env;
use std::path::PathBuf;

use once_cell::sync::OnceCell;

pub struct Context {
    pub current_dir: std::path::PathBuf,
}

impl<'a> Context {
    pub fn new() -> OnceCell<Self> {
        OnceCell::from(Context {
            current_dir: std::env::current_dir().unwrap(),
        })
    }

    pub fn current_dir() -> PathBuf {
        env::current_dir().unwrap()
    }

    pub fn os() -> &'a str {
        env::consts::OS
    }
}
