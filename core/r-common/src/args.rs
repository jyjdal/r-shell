#[derive(Clone)]
pub struct CommandAndArgs {
    pub command: String,
    pub args: Vec<String>,
}

pub type CommandArgs = Vec<String>;
