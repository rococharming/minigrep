use std::env;
use std::fs;

fn main() {

    // 获取所有命令行参数
    // std::env::args() 返回的是 Args，它是一个迭代器类型
    let args = env::args().collect::<Vec<String>>();
    // 注意，命令行的第一个参数（索引为0）是程序名
    let query = &args[1];        // 获取查询词
    let file_path = &args[2];    // 获取文件路径
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // 读取文件内容并打印
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    println!("\nWith text:\n{contents}");

}
