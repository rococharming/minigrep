use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // 获取所有命令行参数
    // std::env::args() 返回的是 Args，它是一个迭代器类型
    let args = env::args().collect::<Vec<String>>();

    // 获取配置
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // 参数不够退出程序
        process::exit(1);
    });

    // 根据配置运行代码
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
