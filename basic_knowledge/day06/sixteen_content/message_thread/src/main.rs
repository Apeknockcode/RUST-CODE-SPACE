// TODO : 使用消息传递来跨线程传递数据
/*
 * TODO : 消息传递
 * 一种流行且能保证安全并发的技术就是: 消息传递
 *  - 线程(或 Actor) 通过彼此发送消息(数据) 来进行通信
 * Go 语言的名言: 不要用共享内存来通信,要用通信来共享内存
 *
 * Rust : Channel(频道) (标准库提供)
 *   - Channel 包含: 发送端,接受端
 *   - 调用发送端的方法 : 发送消息(数据)
 *   - 接受端会检查和接受到达的消息(数据)
 *   - 如果发送端, 接受端中任意一段被丢弃了,那么Channel就“关闭” 了
 * */

//  TODO : 创建Channel
/*
 * 使用 mpsc::channel 函数来创建 Channel
 *  - mpsc 表述 multiple product , single consumer (多个发送端,一个接受端)
 *  - 返回一个Tuple(元组) : 里面元素分别时发送端,接受端
 * 接受端的方法
 *  - recv 方法:阻止当前线程执行,知道Channel 中有值被送来
 *    - 一旦有值接收到,就返回Result<T,E>
 *    - 当发送端关闭,就会收到一个错误
 *
 *  - try_recv 方法,不会阻塞,
 *    - 立即返回 Result <T,E>;
 *      - 有数据到达,返回OK 里面包含着数据
 *      - 否则, 返回错误
 *    - 通常会使用循环调用来检查try_recv的结果
 * */

//  use std::sync::mpsc;
//  use std::thread;

//  fn main(){
//     let (tx,rx) = mpsc::channel(); // tx:发送端 rx :接受端
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });

//     let received =rx.recv().unwrap();
//     println!("Got {:?}",received);
//  }

//  TODO : Channel 和 所有权转移
/*
 * 所有权在消息传递中非常重要,能帮你编写安全,并发的代码
 * */

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel(); // tx:发送端 rx :接受端
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         println!("val is {}",val);
//     });

//     let received = rx.recv().unwrap();
//     println!("Got {:?}", received);
// }

// TODO :发送多个值,看到接受者在等待的一个过程

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel(); // tx:发送端 rx :接受端
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(200));
//         }
//     });

//     for received in rx {
//         println!("Got {:?}", received);
//     }
// }

// TODO : 通过克隆创建多个发送者
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // tx:发送端 rx :接受端

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("1:hi"),
            String::from("1:from"),
            String::from("1:the"),
            String::from("1:thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });



    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got {:?}", received);
    }
}
