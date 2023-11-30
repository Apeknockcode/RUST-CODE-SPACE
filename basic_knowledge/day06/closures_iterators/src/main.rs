use std::thread;
use std::time::Duration;
fn main() {
    // TODO : 闭包 - 使用闭包创建抽象行为
    /*
     * 闭包: 可以捕获其所在环境的匿名函数.
     * 闭包:
     *  - 是匿名函数
     *  - 保存为变量,作为参数
     *  - 可在一个地方创建闭包,然后再另一个上下文中调用闭包来完成运算
     *  - 可从其定义的作用域捕获值
     * */

    // TODO : 例子 ->  生成自定义运动计划的程序

    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // TODO : 闭包的类型推断
    /*
     *  闭包不要求标注参数和返回值类型
     *  闭包通常很短小,只在狭小的上下文中工作,编译器挺长能推断出类型,
     *  可以手动添加类型标注
     *
     *
     * TODO : 函数 和 闭包 的定义语法
     * fn  add_one_v1   (x:u32) -> u32   {x+1};
     * let add_one_v2 = |x:u32| -> u32   {x+1};
     * let add_one_v3 = |x|              {x+1};
     * let add_one_v4 = |x}               x+1;
     *
     * 注意: 闭包的定义最终只会为参数/返回值推断出唯一具体的类型
     *
     * */

    //  TODO : 使用泛型参数 和 fn trait 来存储闭包
    /*
     *  创建一个struct ,他持有闭包及其调用结果
     *   - 只会在需要结果时才执行该闭包
     *   - 可缓存结果
     *  这个模式通常叫做记忆化 或者延迟计算
     *
     * TODO : 如何让Struct 持有闭包
     * struct 的定义需要知道所有字段的类型
     *  - 需要指明闭包的类型
     * 每个闭包实例都有自己的唯一的匿名函数,即使两个闭包签名完全一样
     * 所以需要使用: 泛型 和 Trait Bound
     *
     * TODO : Fn Trait
     * Fn trait 由标准库提供
     * 所有的闭包都至少实现了以下trait 之一:
     *  - Fn
     *  - FnMut
     *  - FnOnce
     * */

    //  TODO : 使用 缓存器 (Cacher) 实现的限制
    /*
     * Cacher 实例假定针对不同的参数 arg , value 方法总会得到同样的值 .
     * 1.可以使用 HasMap 代替单个值:
     *  - key :  arg 参数
     *  - value : 执行闭包的结果
     * 2.只能接受一个u32类型的参数 和 u32类型的返回值
     *
     * */

    //  TODO : 使用闭包捕获环境
    /*
     * 闭包可以捕获他们所在的环境
     * - 闭包可以访问定义他们的作用域内的变量,而普通函数则不能
     * - 会产生内存开销
     * 闭包从所在环境捕获值的方式
     * - 与函数获得参数的三种方式一样.
     *   - 取得所有权 : FnOnce
     *   - 可变借用 : FnMut
     *   - 不可变借用 : Fn
     * 创建闭包时,通过闭包对环境值的使用,Rust 推断出具体使用哪个trait:
     *   - 所有的闭包都实现了 FnOnce
     *   - 没有移动捕获变量的实现了FnMut
     *   - 无需可变访问捕获变量的闭包实现了Fn
     * */

    // let x = 4;

    // let equal_to_x = |z| z == x;

    // let y = 4;
    // assert!(equal_to_x(y));

    // TODO : move 关键字
    /*
     * 使用 move 关键字,可以强制闭包取得它所使用的环境值的所有权,
     * 当传递给新线程以移动数据使其归新线程所有时,此技术最为有用.
     * */

    let move_x = vec![1, 2, 3];
    let test_equal_move_x = move |z| z == move_x;
    // println!("can't use move_x herr: {:?}", move_x); // 报错:value moved into closure here
 
    let test_y = vec![1, 2, 3];
    assert!(test_equal_move_x(test_y));
}

// 模拟复杂的算法
// fn simulated_expensive_calculation(intensity:u32) -> u32{
//     println!("calculating slowly....");
//     // 创建一个睡眠的线程 thread
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// TODO : 定义一个结构体来存储闭包
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T, // 这是存储闭包的
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // TODO : 声明一个闭包

    //  TODO : 手动添加类型的标注
    // let expensive_closure = |num: u32| -> u32 {
    //     // let expensive_closure = |num| {
    //     println!("calculating slowly....");
    //     // 创建一个睡眠的线程 thread
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // TODO : 重构闭包
    let mut expensive_closure = Cacher::new(|num| {
        // let expensive_closure = |num| {
        println!("calculating slowly....");
        // 创建一个睡眠的线程 thread
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today ,do {} pushups", expensive_closure.value(intensity));
        println!("Next ,do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a beak today! Remember to stay hydrated...");
        } else {
            println!(
                "Today,run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}
