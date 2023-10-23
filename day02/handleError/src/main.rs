use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //    panic! 与 不可恢复的错误

    /*
     * Rust 有 panic!宏。当执行这个宏时，程序会打印出一个错误信息，展开并清理栈数据，然后接着退出
     * 当出现 panic 时，
     * 这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作
     * 另一种选择是直接 终止（abort），这会不清理数据就退出程序.那么程序所使用的内存需要由操作系统来清理
     * panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止
     **/

    // 简单的吊用painc!
    // panic!("crash and burn");
    /*
     * 错误信息中:
     * 第一行显示了 panic 提供的信息并指明了源码中 panic 出现的位置
     * 最后两行包含 panic! 调用造成的错误信息。
     * */

    //  使用 panic! 的 backtrace
    let back = vec![1, 2, 3];
    // back[99]; //尝试访问超越 vector 结尾的元素，这会造成 panic!

    // Result 与可恢复的错误
    /*
     * 大部分错误并没有严重到需要程序完全停止执行。
     * 一个函数会因为一个容易理解并做出反应的原因失败。
     *
     * **/

    //  简单示例:
    let file = File::open("Hello.txt");
    // 使用 match 表达式处理可能会返回的 Result 成员
    // let file = match file{
    //     Ok(f) => f,
    //     Err(e) => panic!("Problem opening the file: {:?}", e)
    // };

    // 匹配不同的错误

    // let new_file = match file{
    //     Ok(f) => f,
    //     Err(error)=> match error.kind() {
    //         // 使用不同的方式处理不同类型的错误

    //         // ErrorKind::NotFound，它代表尝试打开的文件并不存在

    //         // File::create 创建文件
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     }
    // };

    // 失败时 panic 的简写：unwrap 和 expect
    /*
     * Result<T, E> 类型定义了很多辅助方法来处理各种情况。其中之一叫做 unwrap.
     * 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
     * 如果 Result 是成员 Err，unwrap 会为我们调用 panic!
     * 
     * expect 与 unwrap 的使用方式一样:返回文件句柄或调用 panic! 宏.
     * expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数，
     * 而不像 unwrap 那样使用默认的 panic! 信息
     * 
     * **/

    // let f_1 = File::open("text.txt").unwrap();

    //  TODO:使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
    let f_1 = File::open("text.txt").expect("打开文件失败");



    /*
    * 传播错误
    * 在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。这被称为 传播（propagating）错误
    */ 
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
/*
 * T 和 E 是泛型类型参数:
 * T 代表成功时返回的 Ok 成员中的数据的类型
 * E 代表失败时返回的 Err 成员中的错误的类型。
 * **/
