// 异步编译 async ， await
use std::fs;

#[tokio::main]
async fn main() {
    // fs::write("README.md", "Hello, world!").unwrap(); // 写入文件
    let files = tokio::fs::read_to_string("README.md").await;
    match files {
        Ok(content) => println!("files is {}", content),
        Err(error) => println!("error is {}", error),
    };

    // tokio::spawn(async{
    //     println!("hello1")
    // }).await;
    // say_hello().await;

    // say_hello().await;
    // say_goodbye().await;
    // 宏 join : 同时并发的执行多个任务
    tokio::join!(say_hello(), say_goodbye());
    // select：选择执行多个任务中的一个
    tokio::select! {
        result = say_hello() => {
            println!("result is {}", result);
        },
        _ = tokio::time::sleep(tokio::time::Duration::from_millis(2000)) => {
            println!("timeout");
        }
    };
}

async  fn say_hello() -> &'static str{
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("hello");
    "Hello TT"
}
async  fn say_goodbye() {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("goodbye");
}
