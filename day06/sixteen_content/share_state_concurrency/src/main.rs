// TODO : 使用共享来实现并发
/*
 * Go 语言的名言.不要用共享内存来通信,要用通信来共享内存
 *
 * Rust 支持通过共享状态来实现并发
 * Channel 类似单所有权:一旦将值的所有权转移至Channel,就无法使用它了
 * 共享内存并发类似多所有权:多个线程可以同时访问一块内存
 *
 * */

//  TODO : 使用Mutex来每次只允许一个线程来访问数据
/*
 * Mutex  是 mutual exclusion (互斥锁)的简写
 * 在同一时刻,mutex只允许一个线程来访问某些数据
 *
 * 想要访问数据:
 *   - 线程必须首先获取互斥锁(lock)
 *     - lock 数据结构是 mutex 的一部分,他能跟踪谁对数据拥有独立访问权
 *   - mutex 通常被描述为: 通过锁定系统来保护它所持有的数据
 * TODO : Mutex 的两条规则
 *   - 在使用数据之前,必须尝试获取锁(lock)  .
 *   - 使用完 mutex 所保护的数据,必须对数据进行解锁,以便其它线程可以获取锁.
 * TODO : Mutex<T> 的API
 *   - 通过Mutex::new(数据) 来创建Mutex<T>
 *     - Mutex<T> 是一个智能指针
 * 访问数据前,通过lock 方法来获取锁
 *   - 会阻塞当前线程
 *   - lock可能会失败
 *   - 返回的是 MutexGuard (智能指针,实现了 Deref 和 drop)
 * */

// use std::sync::Mutex;
// fn main() {
//     let m = Mutex::new(5);
//     {
//         // m.lock() 获取锁
//         let mut num = m.lock().unwrap(); // 阻塞当前线程
//         *num = 6;
//     }
//     println!("m= {:?}", m);
// }

// TODO : 多线程共享
// TODO : 多重线程的多种所有权
// TODO : 使用Arc<T> 来进行原子引用计数
/*
 * Arc<T> 和 Rc<T> 类似,他可以用于并发情景
 * A: atomic 原子的
 * 为什么所有的基础类型都不是原子的,为什么标准库类型不默认使用Arc<T>?
 *   - 需要性能作为代价
 * Arc<T> 和 Rc<T> 的API是相同的
 *  
 * */ 

use std::rc::Rc;
use std::sync::{Mutex,Arc};
use std::thread;
fn main() {
    let counter = Arc::new(Mutex::new(0)); // 计数器
    let mut handles = vec![];

    for _ in 0..10 {
        let counter  = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }
    println!("result : {}", *counter.lock().unwrap())
}



// TODO : RefCell<T> / Rc<T> vs Mutex<T> / Arc<T>
/*
 * Mutex<T> 提供了内部可变性 和 Cell 家族一样
 * 我们使用RefCell<T> 来改变Rc<T> 里面的内容
 * 我们使用Mutex<T> 来改变Arc<T> 里面的内容
 * 注意: Mutex<T> 有死锁的风险
 * */ 