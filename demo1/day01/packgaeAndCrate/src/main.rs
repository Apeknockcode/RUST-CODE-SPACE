// use std::collections::HashMap;
use rand::Rng;
// 通过 glob 运算符将所有的公有定义引入作用域
use std::collections::*;
// mod lib;


fn main() {
    println!("Hello, world!");
    // crate 是一个二进制项或者库。
    // crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块

    //  包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件

    /*
     * 因为 Cargo 遵循的一个约定:
     *  src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。
     *  crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。
     *  ------
     *  如果一个包同时含有 src/main.rs 和 src/lib.rs，
     *  则它有两个 crate：一个库和一个二进制项，且名字都与包相同
     *  ------
     *  一个 crate 会将一个作用域内的相关功能分组到一起.使得该功能可以很方便地在多个项目之间共享
     *
     * */

    //  第一章:定义模块来控制作用域与私有性
    /*
     * 将讨论模块和其它一些关于模块系统的部分
     *   - 用来将路径引入作用域的 use 关键字
     *   - 使项变为公有的 pub 关键字
     *   - 讨论 as 关键字、外部包和 glob 运算符
     */

    // 模块 让我们可以将一个 crate 中的代码进行分组.以提高可读性与重用性
    // 模块还可以控制项的 私有性，即项是可以被外部代码使用的（public），还是作为一个内部实现的内容，不能被外部代码使用

    //  示例:通过执行 cargo new --lib restaurant，来创建一个新的名为 restaurant 的库/

    // 第二章:路径用于引用模块树中的项
    /*
     *如果我们想要吊用一个函数.我们需要知道他的路径
     *路径有两种形式:
     * (-)绝对路径 :从crate根部开始.以crate名或者字面量crate开头
     * (二)相对路径 : 从当前模块开始,以self,super或者当前模块的标识符开始/
     *
     * 绝对路径和相对路径都会跟一个或者多个双冒号(::)分割标识符
     *
     * Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。
     * 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项
     * 你还可以通过使用 pub 关键字来创建公共项，使子模块的内部部分暴露给上级模块。
     */

    //第三章: 使用super起始的相对路径
    /*
     * 我们还可以使用super 开头来构建从父模块开始的相对路径.
     * */

    let mut map = HashMap::new();
    map.insert(1, 3);
    println!("map is value {:?}", map);

    // 使用外部包
    let  secret_number = rand::thread_rng().gen_range(1..23);
    println!("secret_number is {}", secret_number);


    // hosting::add_to_waitlist();
    
}
