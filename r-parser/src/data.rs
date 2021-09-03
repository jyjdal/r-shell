// Parser 解析完命令之后返回一个数组
pub struct CommandAndArgs<'a> {
    pub command: &'a str,
    pub args: Vec<CommandArg<'a>>,
}

#[derive(Copy, Clone)]
pub struct CommandArg<'a> {
    pub tag: Option<&'a str>,
    pub value: ArgValue,
}

impl CommandArg<'_> {
    #[allow(unused)]
    fn new_bool_value(tag: &str, value: bool) -> CommandArg {
        CommandArg {
            tag: Option::Some(tag),
            value: ArgValue::Bool(value),
        }
    }
}

#[derive(Copy, Clone)]
pub enum ArgValue {
    Bool(bool),
}
