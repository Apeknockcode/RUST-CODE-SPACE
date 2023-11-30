fn main() {
    /*
     * vector 用来储存一系列的值
     * vector 只能储存相同类型的值。
     * vector 是用泛型实现的
     */

    //  新建一个空的vector 来储存i32 类型的值
    let mut v: Vec<i32> = Vec::new();
    // 更新Vector -> 使用 push 方法向 vector 增加值
    v.push(3);
    v.push(4);
    println!("v is {:?}", v);
    // 为了方便 Rust 提供了 vec! 宏。这个宏会根据我们提供的值来创建一个新的 Vec
    let v1 = vec![1, 2, 3, 4];
    println!("v1 is {:?}", v1);

    {
        // 作用域
        let v2 = vec![1, 3, 4, 5];
        //  v2  存在
    } // <- 这里v 离开作用域并被丢弃
      // 当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理。

    // 读取 vector 的元素
    let v3 = vec![2, 3, 4, 5];
    let second: &i32 = &v3[2];
    println!("The second element is {}", second);

    match v3.get(2) {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element"),
    }

    // 遍历 vector 中的元素
    for i in &v3 {
        println!("every element is {}", i);
    }

    let mut v4 = vec![100, 23, 343, 22];
    for i in &mut v4 {
        println!("element is {}", *i);
        *i += 50
    }
    println!("result is {:?}", v4);

    // 使用枚举来储存多种类型
    // 提到 vector 只能储存相同类型的值
    // 枚举的成员都被定义为相同的枚举类型,所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举


    // 定义一个枚举，以便能在 vector 中存放不同类型的数据
    let row = vec![
        Spr::Int(3),
        Spr::Float(1.3),
        Spr::Text(String::from("blue")),
    ];
    // dbg!(row)
    println!("row is {:#?}", row);
}

#[derive(Debug)]
enum Spr {
    Int(i32),
    Float(f64),
    Text(String),
}
