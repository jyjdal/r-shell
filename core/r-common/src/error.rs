pub enum ShellError {
    CannotOpenFile(String),
    CannotReadFile(String),
    PathNotSpecified,
    FileNotSpecified,
    PathNotExist,
    ParseError(String),
}
