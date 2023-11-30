// 用关键字 mod 定义一个模块.指定模块的名字.并用大括号包围模块的主体.我们可以用模块mod 来包含其他模块mod
// 模块中也可以包含其他项，比如结构体、枚举、常量、trait，

// // 使用 pub 关键字暴露路径
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn server_order() {}

//         fn take_payment() {}
//     }
// }

// // 使用 use 关键字将名称引入作用域
// // use crate::front_of_house::hosting;
// use front_of_house::hosting;

// // 创建惯用的 use 路径
// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();

//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();

//     add_to_waitlist();
//     add_to_waitlist();
//     add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // 使用以 super 开头的相对路径从父目录开始调用函数
//         super::serve_order()
//     }

//     fn cook_order() {}

//     // 创建公有的结构体和枚举
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant2() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// }

// 两个具有相同名称但不同父模块的 Result 类型引入作用域，以及如何引用它们
// use std::fmt::Result;
// use std::fs::File;
// use std::io::Read;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
//     // ()
// }

// fn function2() -> IoResult<()> {
//     // --snip--
//     // return IoResult::
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }
// }

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

}


 
