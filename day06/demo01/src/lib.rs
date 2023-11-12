use std::fmt::{Debug, Display};
pub trait Summary {
    // 提取 摘要
    // fn summarize(&self) -> String;
    /*
     *  默认实现
     * 默认实现的方法可以调用tarit 中其他的方法,即使这些方法没有默认实现.
     *
     * 注意: 无法从方法的重写实现里面调用默认实现
     * */

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more information)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}", self.headline)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}

// TODO : 在类型上实现trait
/*
 * 与为类型实现方法类似.
 * 不同之处:
 *  - impl  Xxxx for 类型名 {...}
 *
 * */

// TODO : trait 作为参数
/*
 * impl trait  语法: 适用于简单情况
 * trait bound 语法: 可用于复杂情况
 *  - impl Trait 语法是Trait bound 的语法糖
 *
 * 使用 + 指定多个trait bound
 * trait bound 使用where子句
 *  - 在方法签名后执行where 子句
 *
 * */

// TODO :  impl trait  语法: 适用于简单情况
pub fn notify1(item: impl Summary) {
    println!("{}", item.summarize());
}

// TODO : trait bound 语法: 可用于复杂情况

pub fn notify_other<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

pub fn notify1_other(item: impl Summary, item2: impl Summary) {
    println!("{},{}", item.summarize(), item2.summarize());
}

pub fn notify_other2<T: Summary>(item: T, item2: T) {
    println!("{},{}", item.summarize(), item2.summarize());
}

// TODO : 使用 + 指定多个trait bound
pub fn notify2(item: impl Summary + Display) {
    println!("{}", item.summarize());
}

pub fn notify_other3<T: Summary + Display>(item: T, item2: T) {
    println!("{},{}", item.summarize(), item2.summarize());
}

// TODO : trait bound 使用where子句
pub fn notify_other4<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("Breaking news ! {} ", a.summarize())
}

pub fn notify_other5<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking news ! {} ", a.summarize())
}

// TODO : 实现trait 作为返回类型
/*
 * impl trait  语法
 * 注意 : impl trait 只能返回确定的同一种类型,返回可能不同类型的代码会报错
 * */

fn return_parameters(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("news Article is headline"),
        content: String::from("new Article is content"),
        author: String::from("new Article is author"),
        location: String::from("new Article is location"),
    }
}

// TODO : 使用Trait Bound 有条件的实现方法
/*
 * 在使用泛型类型参数的 impl 块上使用trait bound ,我们可以有条件的为实现了特定trait 的类型来实现方法.
 * 也可以为了实现其他的trait 任意类型有条件的实现某个trait
 * 为了满足 trait Bound 的所有类型上实现trait 叫做覆盖实现  
 *
 * */

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("the largest member is x = {}",self.x);
        }else{
            println!("the largest member is y = {}",self.y);
        }
    }
}
