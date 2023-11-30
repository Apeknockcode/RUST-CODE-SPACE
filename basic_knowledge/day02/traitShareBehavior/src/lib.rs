// 概念 Summary trait 的定义
// Summary trait 定义，它包含由 summarize 方法提供的行为
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 为类型实现 trait
// NewsArticle 结构体上 Summary trait 的一个实现，
// 它使用标题、作者和创建的位置作为 summarize 的返回值。
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// 接着是 for 和需要实现 trait 的类型的名称
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.location, self.author)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 接着是 for 和需要实现 trait 的类型的名称
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait 作为参数

// 我们可以传递任何 NewsArticle 或 Tweet 的实例来调用 notify。
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}



// Trait Bound 语法
pub fn notifyother<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 返回实现了trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}