use std::env;
use std::path::PathBuf;

use once_cell::sync::Lazy;

pub struct Context {
    pub current_dir: PathBuf,
}

pub static mut CONTEXT: Lazy<Context> = Lazy::new(|| {
    let c = Context {
        current_dir: env::current_dir().unwrap()
    };
    c
});
