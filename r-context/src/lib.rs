use std::env;
use std::path::PathBuf;

pub struct Context {}

impl<'a> Context {
    pub fn new() -> Self {
        Context {}
    }

    pub fn current_dir() -> PathBuf {
        env::current_dir().unwrap()
    }

    pub fn os() -> &'a str {
        env::consts::OS
    }
}
