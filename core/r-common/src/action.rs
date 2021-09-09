pub enum CommandAction {
    // 退出命令行
    Exit(i32),
    // 清屏
    ClearHost,
    // 更换当前所在目录
    ChangePath(String),
}
