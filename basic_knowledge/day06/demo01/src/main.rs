use demo01::Summary;
use demo01::Tweet;
use demo01::NewsArticle;
fn main() {
    // TODO : Trait
    /*
     * trait  告诉Rust编译器:
     *  - 某种类型具有哪些并且可以与其他类型共享的功能
     * trait : 抽象的定义共享行为
     * trait bounds (约束): 泛型类型参数指定为实现特定行为的类型
     *
     *
     * */

    //   TODO : 定义一个trait

    // tarit 的定义: 把方法签名放在一起,来定义实现某种目的所必需的一组行为,
    /*
     *  关键字 : trait
     * 只有方法签名,没有具体实现
     * trait 可以有多个方法,每一方法签名占一行,以;为结尾
     * 实现该trait的类型必须提供具体的实现方法
     */

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // 

    let article = NewsArticle{
        headline:String::from("Penguins win the stanley Cup Championship !"),
        content:String::from("The Pittsburgh Penguins once again the best hockey team in the NHL"),
        author:String::from("Iceburgh"),
        location:String::from("Pittsburgh ,PA,USA")
    };

    println!("2 new tweet : {}",article.summarize())
}
