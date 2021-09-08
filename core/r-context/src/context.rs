use std::path::PathBuf;

#[derive(Clone)]
pub struct Context {
    pub current_dir: PathBuf,
}
