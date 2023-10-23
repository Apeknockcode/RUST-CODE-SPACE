fn main() {
    // println!("Hello, world!");
    // 使用字符串存储 UTF-8 编码的文本
    // 第一章:什么是字符串？
    /*
     * Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str
     * 它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。
     * String 和字符串 slice 都是 UTF-8 编码.
     *
     * **/

    //  新建字符串
    let mut s = String::new(); // 新建一个空的String
                               // 接着我们可以向其中装载数据
                               // let data = "init content";
                               // let s = data.to_string();

    // 该方法也可直接用于字符串字面量：
    let s = "initial contents".to_string();

    println!("result is {}", s);

    // 使用 to_string 方法从字符串字面量创建 String

    // 以使用 String::from 函数来从字符串字面量创建 String ==>等同于使用 to_string。
    let s_1 = String::from("init content");
    println!("result is {}", s_1);

    // 更新字符串 --> 使用 push_str 方法向 String 附加字符串 slice ,并不需要获取参数的所有权
    let mut s_2 = String::from("foo");
    s_2.push_str("7879");
    println!("result is {}", s_2);

    let mut s_3 = String::from("lo");
    s_3.push('l'); // 使用 push 将一个字符加入 String 值中
    println!("result is {}", s_3);

    // 使用 + 运算符或 format! 宏拼接字符串
    let s1 = String::from("Hello");
    let s2 = String::from(",world!");
    // let s3 = s1 + &s2; // 使用 + 运算符将两个 String 值合并到一个新的 String 值中
    let s4 = add(s1, s2);
    // println!("result is {}", s3);
    println!("result is {}", s4);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("result is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("result is {}", s);

    // 索引字符串
    let s1 = String::from("hello");
    // let h = s1[0];  // 报错 : Rust 的字符串不支持索引

    // 先聊一聊 Rust 是如何在内存中储存字符串的。
    // 内部表现
    //  Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
    let len = String::from("Hola").len();
    println!("len is {}", len);

    // 字符串 slice
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("result is {}", s);

    // 遍历字符串的方法
    for c in "नमस्ते".chars() {
        println!("{}", c); // char 类型的值
    }

    for c in "123434".bytes() {
        println!("{}", c); // 返回每一个原始字节
    }
}

fn add(s1: String, s2: String) -> String {
    return s1 + &s2;
}
