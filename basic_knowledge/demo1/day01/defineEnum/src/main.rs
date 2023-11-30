
// // 定义地址类型
// #[derive(Debug)]
// enum IpAddrKind {
//     v4,
//     v6,
// }

// fn main() {
//     println!("定义枚举");
//     // 枚举值
//     // 注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。
//     let four = IpAddrKind::v4;
//     let six = IpAddrKind::v6;

//     route(four);
// }

// // 定义获取任何IpAddrKind
// fn route(ip_type: IpAddrKind){}

// 结构体 与 枚举 结合
#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

// IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值：
#[derive(Debug)]
enum IpAddrKind2 {
    v4(String),
    v6(String),
}
// 将 IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 将成员中的地址数据嵌入到了两个不同形式的结构体中
#[derive(Debug)]
struct IpV4Addr {
    address: String,
    kind: String,
}
#[derive(Debug)]
struct IpV6Addr {
    address: String,
    kind: String,
}

#[derive(Debug)]
enum IpAddr2 {
    v4(IpV4Addr),
    v6(IpV6Addr),
}
// 定义一个枚举 其每个成员都存储了不同数量和类型的值
/*
*Quit 没有关联任何数据。
*Move 包含一个匿名结构体。
*Write 包含单独一个 String。
*ChangeColor 包含三个 i32。
*/
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 定义不同的结构体- 类单元结构体
#[derive(Debug)]
struct QuitMessage;
#[derive(Debug)]
struct MoveMessage {
    x:i32,
    y:i32
}
#[derive(Debug)]
struct WriteMessage(String);
#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32);

impl Message{
    fn call(&self){
        println!("Message : {:?}", self);
    }
}

#[derive(Debug)]
enum Option<T>{
    Some(T),
    None
}

fn main() {
    // 实例1
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    // 实例2
    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    let home1 = IpAddrKind2::v4(String::from("127.0.0.1"));
    let loopback1 = IpAddrKind2::v6(String::from("::1"));
    // dbg!(home1);
    // dbg!(loopback1);
    println!("homeAddr is {:?}", home1);
    println!("homeAddr is {:?}", loopback1);

    let home2 = IpAddr1::V4(127, 0, 0, 1);

    let IpV4AddrType = IpV4Addr {
        kind: String::from("123"),
        address: String::from("::1"),
    };
    let hom3 = IpAddr2::v4(IpV4AddrType);
    println!("homeAddr is {:?}", hom3);

    // 使用
    let s = Message::Write(String::from("Write"));
    s.call();


    // Option 枚举和其相对于空值的优势
    /*
     * Option 是标准库定义的另一个枚举,即一个值要么有值要么没值
     * 空值（Null ）是一个值，它代表没有值。在有空值的语言中，变量总是这两种状态之一：空值和非空值。
     * */ 


     let some_number = Option::Some(5);
     let some_string = Option::Some("a string");

    //  如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的
     let absent_number: Option<i32> = Option::None;


    let x: i8 = 5;
    
    let y: Option<i8> = Option::Some(5);
    // let sum = x + y; //错误信息意味着 Rust 不知道该如何将 Option<i8> 与 i8 相加，因为它们的类型不同。



    // 总的来说，为了使用 Option<T> 值，需要编写处理每个成员的代码。
    // 你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T。
    // 也希望一些代码在值为 None 时运行，这些代码并没有一个可用的 T 值。
    // match 表达式就是这么一个处理枚举的控制流结构：它会根据枚举的成员运行不同的代码，
    // 这些代码可以使用匹配到的值中的数据。
}
