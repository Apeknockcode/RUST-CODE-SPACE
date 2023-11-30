// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             //  运行多个代码
//             println!("Lucky Penny!");
//             return 1;
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// // 绑定值的模式
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// #[derive(Debug)]
// enum Coin1 {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents1(coin: Coin1) -> u8 {
//     match coin {
//         Coin1::Penny => {
//             //  运行多个代码
//             println!("Lucky Penny!");
//             return 1;
//         }
//         Coin1::Nickel => 5,
//         Coin1::Dime => 10,
//         Coin1::Quarter(state) => {
//             println!("State quarter from {:#?}", state);
//             return 25;
//         }
//     }
// }

// fn main() {
//     println!("match 控制流运算符");
//     // Rust 有一个叫做 match 的极为强大的控制流运算符，
//     // 它允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码

//     /*
//      * 可以把 match 表达式想象成某种硬币分类器：
//      * 硬币滑入有着不同大小孔洞的轨道，每一个硬币都会掉入符合它大小的孔洞。
//      * 同样地，值也会通过 match 的每一个模式，并且在遇到第一个 “符合” 的模式时，
//      * 值会进入相关联的代码块并在执行中被使用。
//      */
//     //  绑定值的模式
//     // 匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值->这也就是如何从枚举成员中提取值的

//     // 示例:让我们修改枚举的一个成员来存放数据
// }

// ----------------------------------------------------------------

// 匹配Options<T>
// 匹配 Some(T)
// 匹配是穷尽的
/*
*我们在之前的部分中使用 Option<T> 时，是为了从 Some 中取出其内部的 T 值；
*我们还可以像处理 Coin 枚举那样使用 match 处理 Option<T>！只不过这回比较的不再是硬币，
*而是 Option<T> 的成员，但 match 表达式的工作方式保持不变。
 * */

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }
// fn main() {
//     let five = Option::Some(5);
//     println!("five is {:?}", five);
//     let six = plus_one(five);
//     println!("six is {:?}", six);
//     let none = plus_one(None);
//     println!("none is {:?}", none);
// }

// ----------------------------------------------------------------

// 通配模式 和 _ 占位符

// 例子，我们希望对一些特定的值采取特殊操作，而对其他的值采取默认操作.
/*
*想象我们正在玩一个游戏，
*如果你掷出骰子的值为 3，角色不会移动，而是会得到一顶新奇的帽子。
* 如果你掷出了 7，你的角色将失去新奇的帽子。对于其他的数值，你的角色会在棋盘上移动相应的格子。
*/

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // 采用占位符
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
