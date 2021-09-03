// Parser 解析完命令之后返回一个数组
#[derive(Clone)]
pub struct CommandAndArgs<'a> {
    pub command: &'a str,
    pub args: Vec<CommandArg<'a>>,
}

impl CommandAndArgs<'_> {
    pub fn to_vec(self) -> Vec<String> {
        let mut res = Vec::<String>::new();
        for arg in self.args {
            if let Some(t) = arg.tag {
                let mut v = String::from("-").to_string();
                v.push_str(t);
                res.push(String::from(v.trim()));
            } else {
                if let ArgValue::String(v) = arg.value {
                    res.push(String::from(v.trim()));
                }
            }
        }
        res
    }
}

#[derive(Clone)]
pub struct CommandArg<'a> {
    pub tag: Option<&'a str>,
    pub value: ArgValue,
}

impl CommandArg<'_> {
    pub fn new_bool_value(tag: &str, value: bool) -> CommandArg {
        CommandArg {
            tag: Option::Some(tag),
            value: ArgValue::Bool(value),
        }
    }
}

#[derive(Clone)]
pub enum ArgValue {
    Bool(bool),
    String(String),
}
