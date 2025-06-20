// TODO: 宏
//  声明宏 ： macro_rules!
//  过程宏 ： derive |
//  - 派生宏：#[derive(Debug)]
//  - 宏函数： sql!(SELECT * FROM posts)
//  - 宏属性： #[get("/users")]

// 创建宏
macro_rules! say_hello{
    () => {
        println!("hello");
    };
    ($name:expr) => {
        println!("hello {}", $name);
    };
    ($name:expr, $age:expr) => {
        println!("hello {} {}", $name, $age);
    };
}
macro_rules! nameof{
    ($name:ident) => {
        stringify!($name)
    };
}
struct Person;
fn main() {
    println!("Hello, world!");
    say_hello!();
    say_hello!("TT");
    say_hello!("TT", 18);
    println!("name is {}", nameof!(Person));
}
