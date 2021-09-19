use crate::{CommandArg, CommandArgs};

#[derive(Copy, Clone)]
pub struct LsArgs {
    pub all: bool,
    pub depth: i32,
}

const LS_ARGS_DEFAULT: LsArgs = LsArgs {
    all: false,
    depth: 1,
};

impl CommandArgs for LsArgs {
    fn build(args: Vec<CommandArg>) -> Self {
        let mut ret = LS_ARGS_DEFAULT;

        for arg in args {
            if arg.tag == Some("a") {
                ret.all = true;
            }
        }

        ret
    }
}
