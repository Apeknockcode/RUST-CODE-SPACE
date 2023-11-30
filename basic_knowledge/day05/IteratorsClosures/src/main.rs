use std::thread;
use std::time::Duration;

use std::collections::HashMap;

fn main1() {
    // TODO : 闭包 : 可以捕获环境的匿名函数
    /*
     * 闭包式可以保存进变量或者参数传递给其他函数的匿名函数.
     *
     * 可以在一个地方创建闭包.然后在不同上下文执行闭包运算.
     *
     * 不同的函数,闭包允许捕获调用者作用域中的值.
     *
     * 我们将展示闭包的这些功能如何复用代码和自定义行为
     */

    // TODO : 使用闭包创建行为的抽象
    /*
     * simulated_expensive_calculation 函数来模拟吊用假定的算法.
     */

    // TODO :  example1:
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // TODO : 重构使用闭包储存代码

    /***
     * 闭包定义是 expensive_closure 赋值的 = 之后的部分
     * 闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；
     * 之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似
     *
     *
     * 这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔
     *
     * */
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(5);
}

// 函数来模拟吊用假定的算法. -> 一个用来代替假定计算的函数，它大约会执行两秒钟
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// main 函数包含了用于 generate_workout 函数的模拟用户输入和模拟随机数输入
fn generate_workout(intensity: u32, random_number: u32) {
    // 程序的业务逻辑，它根据输入并调用 simulated_expensive_calculation 函数来打印出健身计划
    // if intensity < 25 {
    //     println!(
    //         "Today, do {} pushups!",
    //         simulated_expensive_calculation(intensity)
    //     );
    //     println!(
    //         "Next, do {} situps!",
    //         simulated_expensive_calculation(intensity)
    //     );
    // } else {
    //     if random_number == 3 {
    //         println!("Take a break today! Remember to stay hydrated!");
    //     } else {
    //         println!(
    //             "Today, run for {} minutes!",
    //             simulated_expensive_calculation(intensity)
    //         );
    //     }
    // }

    // TODO : 使用函数重构

    // 将 simulated_expensive_calculation 调用提取到一个位置，并将结果储存在变量 expensive_result 中
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }

    // TODO : 重构使用闭包储存代码

    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(5);

    // TODO :  闭包类型推断和标注

    /***
     * 闭包不要求像 fn 函数那样在参数和返回值上注明类型
     * 函数中需要类型标注是因为他们是暴露给用户的显式接口的一部分.
     * 严格的定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的
     * 但是闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。
     *
     *
     * 闭包通常很短，并只关联于小范围的上下文而非任意情境。
     * 在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样。
     *
     * 示例 13-5 中定义的闭包标注类型将看起来像示例 13-7 中的定义
     * */

    //  为闭包的参数和返回值增加可选的类型标注
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // 展示了闭包语法如何类似于函数语法，除了使用竖线而不是括号以及几个可选的语法之外：

    // 第一行展示了一个函数定义
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    // 而第二行展示了一个完整标注的闭包定义
    // 因为闭包体只有一行。这些都是有效的闭包定义，并在调用时产生相同的行为
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;
}

fn main2() {
    // TODO :使用迭代器处理元素序列
    /*
     * 迭代器模式允许你对一个序列的项进行某些处理
     * 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
     * 当使用迭代器时，我们无需重新实现这些逻辑
     *
     *
     * 在 Rust 中，迭代器是 惰性的（lazy）,这意味着在调用方法使用迭代器之前它都不会有效果
     * */

    // 示例 13-13 中的代码通过调用定义于 Vec 上的 iter 方法在一个 vector v1 上创建了一个迭代器
    // 创建一个迭代器
    let v1 = vec![1, 3, 4];
    let v1_iter = v1.iter();

    // 在一个 for 循环中使用迭代器
    for val in v1_iter {
        // 我们可能会使用一个从 0 开始的索引变量，
        // 使用这个变量索引 vector 中的值，并循环增加其值直到达到 vector 的元素数量
        println!("Got: {}", val);
    }
}

pub trait Iterator {
    // 新语法：type Item 和 Self::Item， 定义了 trait 的 关联类型

    type Item; // Item 类型将是迭代器返回元素的类型

    // next 是 Iterator 实现者被要求定义的唯一方法
    fn next(&mut self) -> Option<Self::Item>;
    // 此处省略了方法的默认实现
}
fn main3() {
    // TODO : Iterator trait 和 next 方法
    // 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait

    /*
     * 如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。
     * 类似的，如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter。
     * */
}

fn main4() {
    // TODO : 消费迭代器的方法

    /*
     * 这些调用 next 方法的方法被称为 消费适配器,因为调用他们会消耗迭代器.
     * */

    //  TODO : 一个消费适配器的例子是 sum 方法.
    /*
     *  这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。
     *  当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和
     * */
}

fn main5() {
    // TODO  : 产生其他迭代器的方法
    /*
     * Iterator trait 中定义了另一类方法，被称为 迭代器适配器
     *  我们允许我们将当前迭代器变为不同类型的迭代器。
     * 可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，
     * 必须调用一个消费适配器方法以便获取迭代器适配器调用的结果
     * */

    //  示例 13-17 展示了一个调用迭代器适配器方法 map 的例子 ,
    // 该 map 方法使用闭包来调用每个元素以生成新的迭代器

    let v1: Vec<i32> = vec![1, 2, 3];
    // 调用迭代器适配器 map 来创建一个新迭代器
    // v1.iter().map(|x| x + 1); // 报错:  unused `std::iter::Map` which must be used: iterator adaptors are lazy
    // and do nothing unless consumed

    // 调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // 因为 map 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作.
    assert_eq!(v2, vec![1, 2, 3]);
}

fn main6() {
    //  TODO : 使用闭包获取环境
    // 让我们展示一个通过使用 filter 迭代器适配器和捕获环境的闭包的常规用例
    // 迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包
    /*
     * 如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。
     * 如果闭包返回 false，其值不会包含在结果迭代器中。
     * */


    //  示例 13-19 展示了使用 filter 和一个捕获环境中变量 shoe_size 的闭包，
    //  这样闭包就可以遍历一个 Shoe 结构体集合以便只返回指定大小的鞋子：


}


fn main() {
    // TODO : 实现 Iterator trait 来创建自定义迭代器
    /*
     * 可以通过在 vector 上调用 iter、into_iter 或 iter_mut 来创建一个迭代器
     *  
     * */ 


    //  let test  = "Hello world";
    //  let mut map = HashMap::new();
    //  for word  in test.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    //  }
    //  println!("{:#?}",map)

    // let v = vec![1,2,3,4,5];
    // v[222];


    //  TODO : Result 枚举

    

}


