use std::env;
use std::fs;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // println!("输入的指令: {:?}", args);
    // // let query = &args[1];
    // let filename = &args[2];

    // // println!("Searching for {}", query);
    // println!("In file {}", filename);

    // // 读取文件
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // println!("With text : \n {}", contents); // cargo run the poem.txt

    // TODO :  提取参数解析器
    // let args:Vec<String> = env::args().collect();
    // let (query , filename) = parse_config(&args);

    // TODO :  组合配置值

    // let args: Vec<String> = env::args().collect();
    // let config = parse_config(&args);
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // let contents =
    //     fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // TODO : 创建一个config构造函数
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
}
// 提取参数解析器
// fn parse_config(args: &[String]) ->(&str,&str) {
//     let query = &args[0];
//     let filename = &args[1];
//     (query, filename)
// }

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[0].clone();
        let filename = args[1].clone();
        Config { query, filename }
    }

    /*
     * 这里将 main 中调用 parse_config 的地方更新为调用 Config::new。
     * 我们将 parse_config 的名字改为 new 并将其移动到 impl 块中，
     * 这使得 new 函数与 Config 相关联。再次尝试编译并确保它可以工作。
     */
}

// 重构 parse_config 返回一个 Config 结构体实例
/**
 * 新定义的结构体 Config 中包含字段 query 和 filename。
 * parse_config 的签名表明它现在返回一个 Config 值
 * */
fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        // panic!("not enough arguments");
        return Err("not enough arguments");
    }

    let query = args[0].clone();
    let filename = args[1].clone();
    // 从 Config::new 中返回 Result
    Ok(Config { query, filename })
}
