pub enum ShellAction {
    // 退出命令行
    Exit(i32),
    // 清屏
    ClearHost,
    // 更换当前所在目录
    ChangePath(String),
    // 输出一些内容
    OutputResult(String),
}
