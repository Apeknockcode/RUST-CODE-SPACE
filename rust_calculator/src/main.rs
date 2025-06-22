mod calc;
fn main() {
    println!("欢迎使用Rust 高性能计算器");
    println!("请输入表达式,按回车结束，仅支持+,-,*,/,^,()");
    println!("输入q退出");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "q\n" {
            break;
        }
        match  calc::calculate(&input) {
            Ok(result) => println!("计算结果为:{}", result),
            Err(err) => println!("计算错误:{}", err),
        }
    }
    println!("感谢使用Rust 高性能计算器");
}
