use std::fmt::Display;
fn main() {
    // TODO : 生命周期
    /*
     * Rust的每一个引用都有自己的生命周期
     * 生命周期: 引用保持有效的作用域
     * 大多数情况: 生命周期是隐式的,可被推断的
     * 当引用的生命周期可能以不同的方式互相关联时: 手动标注生命周期.
     *
     *
     * */

    //  TODO : 生命周期 - 避免悬垂引用
    /*
     * 生命周期的主要目标: 避免悬垂引用
     * */

    // {                              //  ---- r值的生命周期 ----- ‘a
    //     let r;                     // ----------------------- /
    //     {                          // ----------------------- /
    //         let x = 5;             // ----------- x 的生命周期标注 'b
    //         r = &x;                // ------------ /
    //     }                          // ----------------------- /
    //     println!("r:{}", r);       // ------------------------/
    // }                              // ------------------------ +

    //  TODO :  借用检查器

    // TODO :  函数中的泛型生命周期
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     // string1.as_str() 将字符串 string1 转化为 字符切片 通过as_str()
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("result:{}", result);

    // TODO :  生命周期标注语法
    /*
     * 生命周期的标注不会改变引用的生命周期长度
     * 当指定了泛型生命周期参数,函数可以接收带有任何生命周期的引用
     * 生命周期的标注,描述了多个引用生命周期的关系,但不影响生命周期
     * 生命周期标注语法:
     *  - 以 ‘ 开头
     *  - 很多人使用 ’a
     * 生命周期标注的位置:
     *  - 在引用的 & 符号后,
     *  - 使用空格将标注和引用的类型分开
     * 生命周期标注 - 例子
     *  - &i32  // 一个引用
     *  - &‘a i32 // 带有显示生命周期的引用
     *  - &‘a mut i32 // 带有显示生命周期可变引用
     *
     * 单个生命周期标注本身没有意义.
     * */

    // TODO :  函数签名中的生命周期的标注
    /*
     *  泛型生命周期参数声明在: 函数名和 参数列表之间的 <> 里 // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
     * */

    //  TODO : 深入理解生命周期
    /*
     * 指定生命周期参数的方式依赖于函数所做的事情
     * 从函数返回引用时,返回类型的生命周期参数需要与其中一个参数的生命周期匹配:
     * 如果返回的引用没有指向任何参数,那么它只能引用函数内创建的值:
     *  - 这就是悬垂引用: 该值在函数结束时就走出了作用域
     * */

    //  TODO :  struct 定义中的生命周期标注
    /*
     * Struct 里可包括:
     *  - 自持有的类型
     *  - 引用: 需要在每个引用上添加生命周期标注
     * */

    let novel = String::from("Call me Ishmael.Some years age ........");
    let first_sentence = novel.split(".").next().expect("Couldn't find a");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // TODO : 生命周期省略的三个规则:
    /*
     * 编译器使用3个规则在没有显示标注生命周期的情况下,来确定引用的生命周期:
     *  - 规则1 应用于输入生命周期
     *  - 规则2,3  应用于输出生命周期
     *  - 如果编译器应用完 3个规则之后,仍然有无法确定生命周期的引用 ->  报错
     *  - 这些规则适用于 fn 定义 和 impl 块
     *
     * 规则1 :    每一个引用类型都有自己的生命周期
     * 规则2 :    如果只有一个输入生命周期,那么该生命周期被赋予给所有的输出生命周期
     * 规则3 :    如果有多个输入生命周期,但其中一个是&self 或者 &mut self ,那么self 的生命周期会被赋予给所有的输出生命周期参数
     *
     * */

    //  TODO : 方法定义中的生命周期标注
    /*
     *  在Struct 上使用生命周期实现方法.语法和泛型参数的语法一样
     *  在哪声明和使用生命周期参数,依赖于:
     *  - 生命周期参数是否和字段,方法的参数或返回值有关.
     *  struct 字段的生命周期名:
     *  - 在impl 后声明
     *  - struct 名后使用
     *  - 这些生命周期是struct类型的一部分
     * impl 块内的方法签名中:
     * - 引用必须绑定于 struct 字段引用的生命周期,或者引用是独立也可以
     * - 生命周期省略规则经常使得方法中的生命周期标注不是必须的
     *
     * */

    // TODO :  静态生命周期

    /*
     * ‘static 是一个特殊的生命周期: 整个程序的持续时间.
     *  例如: 所有的字符串字面值都拥有 ’static 生命周期
     *    - let s : &'static str = "I have s static lifetime" ;
     * 为了引用指定 'static 生命周期要三思:
     *    - 是否需要引用在程序整个生命周期内都存活.
     * */

    //  TODO : 泛型参数类型 , Trait Bound , 生命周期
}

//  TODO : 泛型参数类型 , Trait Bound , 生命周期

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attebtion please: {}", announcement);
        self.part
    }
}

//  TODO : 生命周期 'a 的实际生命周期是: x 和 y 两个生命周期中较小的那个 string2
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
