// fn main() {
//     println!("引用与借用");
//     let s1 = String::from("Hello world");
//     let len = calulate_length(&s1); // 这些 & 符号就是 引用，
//     println!("The length of '{}' is {}.", s1, len);

//     /*
//      * 创建一个引用的行为称为 借用borrowing
//      * 注意：与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*。
//      * */
// }

// fn calulate_length(s: &String) -> usize {  // s 是对 String 的引用
//     let len = s.len();
//     return len;
// } // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权

// 可变引用
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
//     println!("ss is {}", s);

//     // 可变引用有一个很大的限制：
//     // 在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败：
//     let mut ss = String::from("Hello world");
//     // let  r1 = &mut ss; // first mutable borrow occurs here

//     // let r2 = &mut ss; // second mutable borrow occurs here
//     // println!("{}, {}", r1, r2);

//     /*
//      * 这个限制的好处是 Rust 可以在编译时就避免数据竞争。
//      * 数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
//      **/

//     // 修改
//     {
//         let r1 = &mut ss;
//         println!("{}", r1);
//     } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
//     let r2 = &mut ss;
//     println!("{}", r2);
//     //  ---------------
//     let r1 = &ss; // 没问题
//     let r2 = &ss; // 没问题
//     println!("{} and {}", r1, r2);
//     // 此位置之后 r1 和 r2 不再使用
//     let r3 = &mut ss; // 没问题
//     println!("{}", r3);
// }
// fn change(some_string: &mut String) {
//     return some_string.push_str(",world");
// }

// 悬垂引用
fn main(){
    println!("悬垂引用"); // 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者
    let reference_to_nothing = dangle();
}

// 错误写法
// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 是一个新字符串
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。

// 正确写法
fn dangle() -> String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。