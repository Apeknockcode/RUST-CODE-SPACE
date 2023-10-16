fn main() {
    println!("所有权规则!");
    /*
     * 首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：
     *  Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
     * 值在任一时刻有且只有一个所有者。
     * 当所有者（变量）离开作用域，这个值将被丢弃。
     * */

    //  变量作用域
    let s = "hello";
    println!("s is {}", s);
    // println!("area is {}",  area);
    {
        // area 在这里无效, 它尚未声明
        let area = "area"; // 从此处起，area 开始有效
        println!("变量作用域 area is {}", area);
    } // 此作用域已结束，area 不再有效
      // println!("area is {} is",area);
      /*
       *  这里有两个重要的时间点：
       *  当 area 进入作用域 时，它就是有效的。
       *  这一直持续到它 离开作用域 为止。
       */

    // String 类型
    // 前面介绍的类型都是已知大小的，可以存储在栈中，并且当离开作用域时被移出栈
    // 不过我们需要寻找一个存储在堆上的数据来探索 Rust 是如何知道该在何时清理数据的。
    // Rust 有第二个字符串类型，String。这个类型管理被分配到堆上的数据，
    // 所以能够存储在编译时未知大小的文本。可以使用 from 函数基于字符串字面量来创建 String，如下：
    let new_string = String::from("hello world");
    // 这两个冒号（::）是运算符，允许将特定的 from 函数置于 String 类型的命名空间
    println!("new_string is {}", new_string);

    // 可以修改此类字符串
    let mut mut_String = String::from("mut String");

    mut_String.push_str(", Hello World");

    println!("mut String is {}", mut_String);

    // 内存与分配
    /*
     * 对于 String 类型，为了支持一个可变，可增长的文本片段，
     * 需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
     * 1.必须在运行时向内存分配器请求内存
     * 2.需要一个当我们处理完 String 时将内存返回给分配器的方法。
     * 当调用 String::from 时，它的实现 (implementation) 请求其所需的内存。
     */

    // Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。
    {
        let new_S = String::from("hello"); // 从此处起，new_S 开始有效
        println!("作用域new_S : {}", new_S); // 使用  new_S
    } // 此作用域已结束，
    let new_S = String::from("World!");
    println!("new_S value 被释放重新声明 : {}", new_S);

    // 变量与数据交互的方式一: 移动 (浅拷贝)
    let x = 5;
    let y = x;
    println!("x is {}", x);
    println!("y is {}", y);

    let s1 = String::from("Hello"); //
    let s2 = s1;
    // 我们将 s1 赋值给 s2，String 的数据被复制了，
    // 这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据
    // 那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝
    // println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    // 变量与数据交互的方式二: 克隆 (深拷贝 - clone())
    let s3 = String::from("Clone");
    let s4 = s3.clone();

    println!("s3 is {}", s3);
    println!("s4 is {}", s4);

    // 只在栈上的数据:拷贝 (copy)
    let ss = 6;
    let yy = ss;
    println!("ss is {},yy is {}", ss, yy);
    /*
     * 如下是一些 Copy 的类型：
     * 所有整数类型，比如 u32
     * 布尔类型，bool，它的值是 true 和 false。
     * 所有浮点数类型，比如 f64。
     * 字符类型，char
     * 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。
     * */

    //  所有权与函数
    let sss = String::from("Hello, world!");
    takes_ownership(sss);
    // println!("{}", sss);

    let xxx = 5;
    makes_copy(xxx); // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("xxx is {}", xxx);

    // 返回值与作用域 - 返回值也可以转移所有权
    let ss1 = gives_ownerShip();
    println!("ss1 is {}", ss1);
    let ss2 = String::from("hello");

    let ss3 = takes_and_gives_back(ss2);
    // println!("ss2 is {}", ss2); // ss2 被移动到
    println!("ss3 is {}", ss3);

    // 变量的所有权总是遵循相同的模式：
    // 将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，
    // 其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

    // 我们可以使用元组来返回多个值
    let sss1 = String::from("Hello world");
    let (sss2, len) = calulate_length(sss1);
    println!("The length of '{}' is {}.", sss2, len);
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，

fn takes_ownership(sss: String) {
    println!("sss: {}", sss);
}
fn makes_copy(sss: i32) {
    println!("sss: {}", sss);
}

fn gives_ownerShip() -> String {
    let some_string = String::from("yours");

    return some_string;
}

fn takes_and_gives_back(s: String) -> String {
    return s;
}

fn calulate_length(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}
