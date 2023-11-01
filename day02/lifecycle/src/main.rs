// 生命周期与 引用有效性

/*
 * Rust 中的每一个引用都有其 生命周期
 * 也就是引用保持有效的作用域.
 * 大部分时候生命周期是隐含并可以推断的.
 * 所以 Rust 需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的。
 * **/

//  TODO : 第一章生命周期避免了悬垂引用
/*
 * 生命周期的主要目标是避免悬垂引用.他会导致程序引用了非预期引用的数据.
 * 例如:他有一个外部作用域和一个内部作用域
 * **/

// fn main() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x;
//             println!("r : {}", r);
//         }
//     }

//     // 因为:尝试使用离开作用域的值的引用,导致 报错 cannot find value `r` in this scope
//     // println!("r : {}", r);

//     // 一个有效的引用，因为数据比引用有着更长的生命周期
//     let r1 = 5;
//     let x1 = r1;
//     println!("x1 : {}", x1);
// }

// TODO: 借用检查器
/***
 * 它比较作用域来确保所有的借用都是有效的
 * */

//  TODO: 函数中的泛型生命周期
// fn main() {
//     let mut string1 = String::from("abcdefghijklmnopqrstuvwxyz");
//     let mut string2 = "xyz";

//     // as_str() 用于将字符串类型转换为字符串切片类型（&str)
//     let result = longest(&mut string1.as_str(), &mut string2);
//     println!("The longest string is {}", result);
// }

// 我们将增加泛型生命周期参数来定义引用间的关系以便借用检查器可以进行分析。
// fn longest(x: &str , y: &str)-> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// };

// TODO: 生命周期标注语法
/*
 * 生命周期标注有着一个不太常见的语法：生命周期参数名称必须以撇号（'）开头
 * 其名称通常全是小写，类似于泛型其名称非常短.---> 'a 是大多数人默认使用的名称
 * 生命周期参数标注位于引用的 & 之后，并有一个空格来将引用类型与生命周期标注分隔开。
 *
 * **/
//  &i32        // 引用
//  &'a i32     // 带有显式生命周期的引用
//  &'a mut i32 // 带有显式生命周期的可变引用

// TODO : 函数签名中的生命周期标注
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// TODO: 看看如何通过传递拥有不同具体生命周期的引用来限制 longest 函数的使用
// fn main() {
//     // let string1 = String::from("long string is long");

//     // {
//     //     let string2 = String::from("xyz");
//     //     // 通过拥有不同的具体生命周期的 String 值调用 longest 函数
//     //     let result = longest(string1.as_str(), string2.as_str());

//     //     println!("The longest string is {}", result);
//     // }

//     // 在这个例子中，string1 直到外部作用域结束都是有效的，
//     // string2 则在内部作用域中是有效的，而 result 则引用了一些直到内部作用域结束都是有效的值。
//     // 借用检查器认可这些代码；它能够编译和运行，并打印出 The longest string is long string is long。

//     //TODO : 另外一个例子，该例子揭示了 result 的引用的生命周期必须是两个参数中较短的那个
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     // 尝试在 string2 离开作用域之后使用 result ,报错
//     println!("The longest string is {}", result);

// }

//  TODO : 深入理解生命周期
/**
 * 指定生命周期参数的正确方式依赖函数实现的具体功能
 *
 * */

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }
// 在这个例子中，我们为参数 x 和返回值指定了生命周期参数 'a，不过没有为参数 y 指定，
// 因为 y 的生命周期与参数 x 和返回值的生命周期没有任何关系。

//综上 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
// 一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。

// TODO : 结构体定义中的生命周期标注

// 一个存放引用的结构体，所以其定义需要生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    // 在 Rust 中，.next() 是一个用于迭代器的方法，用于获取迭代器中的下一个元素。
    // 它返回一个 Option 类型的值，表示下一个元素是否存在。如果存在，则返回 Some(value)，
    // 其中 value 是下一个元素的值；如果不存在，则返回 None。
    let first_sentence = novel.split('.').next().expect("Could  not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 它没有生命周期标注却能编译成功
// 定义了一个没有使用生命周期标注的函数，即便其参数和返回值都是引用
// fn first_word(s: &str) -> &str {
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// TODO : 函数或方法的参数的生命周期被称为 输入生命周期
// TODO : 返回值的生命周期被称为 输出生命周期


// TODO : 编译器采用三条规则来判断引用何时不需要明确的标注
// 第一条规则是每一个是引用的参数都有它自己的生命周期参数
// 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
// 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
// 说明是个对象的方法那么所有输出生命周期参数被赋予 self 的生命周期



//  TODO : 方法定义中的生命周期标注
//  TODO : 静态生命周期
// 这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。
// let s: &'static str = "I have a static lifetime.";

//  TODO : 结合泛型类型参数、trait bounds 和生命周期