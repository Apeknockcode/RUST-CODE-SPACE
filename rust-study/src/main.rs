// // 特征 Trait

// // 文件操作主要包含四个：open 、write、read、close，
// // fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
// //     a + b
// // }
// // fn main() {
// //     println!("add is {}", add(3, 4));
// // }

// pub trait Summary {
//     fn summarize_authors(&self) -> String;

//     //  定义一个默认方法 默认实现的
//     fn summarize(&self) -> String {
//         format!("Read more form {} ...", self.summarize_authors())
//     }
// }
// //  Post 结构体
// // 为类型实现特征
// struct Post {
//     pub title: String, //  标题
//     pub author: String, // 作者
//     pub content: String, // 内容
// }
// // impl Summary for Post {}
// // impl Summary for Post {
// //     fn summarize(&self) -> String {
// //         format!("{} {}", self.title, self.author)
// //     }
// // }

// //  Weibo 结构体

// #[derive(Debug)]
// pub struct Weibo {
//     pub username: String,
//     pub content: String,
// }

// impl Summary for Weibo {
//     // fn summarize(&self) -> String;

//     fn summarize_authors(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// fn main() {
//     // 这个类型上调用特征的方法
//     let post = Post {
//         title: "Rust语言简介".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust棒极了!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "好像微博没Tweet好用".to_string(),
//     };

//     // println!("{}", post.summarize());
//     println!("weibo struct is {:?}",weibo);
//     println!("new weibo: {}", weibo.summarize());
//     // println!("{}", weibo.summarize());
// }

// 使用特征作为函数参数
pub trait Summary {
    fn summarize_authors(&self) -> String;
    //  定义一个默认方法 默认实现的
    fn summarize(&self) -> String {
        format!("Read more form {} ...", self.summarize_authors())
    }
}
// 定义一个函数，使用特征作为函数参数：
// fn notify(item: &impl Summary) { // 实现了Summary特征 的 item 参数
//     println!("Breaking news!  {}", item.summarize())
// }
// 特征约束(trait bound)
// fn notify<T : Summary>(item: &T) { // T: Summary 被称为特征约束。
//     println!("Breaking news!  {}", item.summarize())
// }

// 多重约束
// fn notify<T: Summary + std::fmt::Display>(item: &T) {}

// Where 约束
// fn some_function<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(t: &T, u: &U) {}
// 进阶版本
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug
{
    return 12;
}

// 使用特征约束有条件地实现方法或特征
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
fn main() {}
