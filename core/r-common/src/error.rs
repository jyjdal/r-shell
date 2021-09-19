pub enum ShellError {
    CannotOpenFile(String),
    PathNotSpecified,
    FileNotSpecified,
    PathNotExist,
}
