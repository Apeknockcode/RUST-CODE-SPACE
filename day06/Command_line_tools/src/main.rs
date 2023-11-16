

use Command_line_tools::Config;
use std::env;
use std::process;

fn main() {
    // TODO :  项目实例: - 命令行程序
    /*
     * 12.1 - 接收命令行参数
     * 12.2 -读取文件
     * 12.3 - 重构:该改进模块和错误处理
     * 12.4 - 使用TDD(测试驱动开发) 开发库功能
     * 12.5 - 使用环境变量
     * 12.6 - 将错误消息写入标准错误而不是标准输出
     * */

    // TODO : 读取命令行的参数
    // let args: Vec<String> = env::args().collect();
    // TODO : 打印参数值
    // println!("{:?}", args);
    /*
     * 执行 cargo run 输出: // ["target/debug/Command_line_tools"]
     * 执行 cargo run 123 Abcsd 输出 ["target/debug/Command_line_tools", "123", "Abcsd"]
     * 获取命令行的规则是:
     *  - 获取集合的第一个元素是这个二进制程序文件的名称
     *  - 后面的才是命令行输入的真实的参数
     * */

    //  TODO : 获取用户输入的命令行的真实的参数
    // let query = &args[1];
    // let filename = &args[2];
    // println!("获取用户输入的一个参数: {}", query);
    // println!("获取用户输入的二个参数: {}", filename);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // TODO : 读取文件
    // let contents = fs::read_to_string(config.filename).expect("读取错误");
    // 打印读取的文件
    // println!("读取的文件 : \n {}", contents);
    if let Err(e) = Command_line_tools::run(config) {
        eprintln!("Application error : {}",e);
        process::exit(1);
    }

    // TODO : 重构上面代码 -> 改进模块和错误处理能力
    /*
     * 二进制程序关注点分离的指导性规则
     *  - 将程序拆分为 main.rs 和 lib.rs ,将业务逻辑放入lib.rs
     *  - 当命令行解析逻辑较少时,将它放在main.rs 也行
     *  - 当命令行解析逻辑复杂时,需要将它从main.rs 提取到 lib.rs
     * 经过上述拆分,留在main的功能有:
     *  - 使用参数值调用命令行解析逻辑
     *  - 进行其他配置
     *  - 调用lib.rs 中run 函数
     *  - 处理run 函数可能出现的错误
     * */


    //  TODO : 测试驱动开发
    /*
     * 步骤1 : 编写一个会失败的测试.运行该测试, 确保它是按照预期的原因失败
     * 步骤2 : 编写或修改刚好足够的代码,让新测试通过
     * 步骤3 : 重构刚刚添加或者修改的代码,确保测试会始终通过
     * 步骤4 :返回步骤1 继续. 
     * 
     * */ 

    //  TODO :标准输出 vs 标准错误
    /*
     *  标准输出 : stdout
     *  - printIn! 
     * 标准错误: stderr
     *  - eprintIn! 
     * */ 

}


