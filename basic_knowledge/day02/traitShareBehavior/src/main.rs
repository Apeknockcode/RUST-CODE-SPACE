// trait : 定义共享的行为
/*
* trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能
* 可以通过 trait 以一种抽象的方式定义共享的行为,指定泛型是任何拥有特定行为的类型
* trait 类似于其他语言中常被称为 接口（interfaces）的功能，虽然有一些不同
*/

// 定义trait
/*
* 如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了
* trait 定义是一种将方法签名组合起来的方法,目的是定义一个实现某些目的所必须的行为的集合
*/

mod lib;

use lib::Summary;

fn main() {
    /*
     * 例如:有多个存放了不同类型的属性文本的结构体:NewsArticle 用于存放发生与世界各地的新闻故事,
     * 而结构体Tweet 最多只能存放280个字符的内容,
     *
     * **/
    //  一旦实现了 trait，
    // 我们就可以用与 NewsArticle 和 Tweet 实例的非 trait 方法一样的方式调用 trait 方法了：
    let tweet = lib::Tweet{
        username:String::from("horse_ebooks"),
        content:String::from("of course, as you probably already know, people"),
        reply:false,
        retweet:false,
    };
    println!("1 new tweet: {}", tweet.summarize());


    lib::notify(tweet)
    // let article = lib::NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from("The Pittsburgh Penguins once again are the best
    //     hockey team in the NHL."),
    // };
    
    // println!("New article available! {}", article.summarize());

    
}



// 为类型实现 trait
