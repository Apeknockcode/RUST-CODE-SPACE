// TODO: 多线程
/**
 * 创建线程： thread::spawn(|| {
 *     println!("Hello from a new thread!");
 * });
 *
 * 消息传递:  mpsc::channel()
 *
 * 共享状态：Mutex<T> Arc<T>
 *
 * 线程安全： arc<Mutex<T>> RwLock<T>
 *
 *
 * */

use std::thread;
use std::time::Instant;
use std::any;
use std::sync::{ Arc, Mutex };
use std::sync::mpsc;

const TOTAL: u64 = 1000000000u64;
fn main() {
    println!("学习线程");
    let person = Person { name: String::from("张三") };
    // // 创建线程
    // // 使用 move 会强制闭包直接获取变量的所有权，即使原本可以通过借用捕获。
    // let handle = thread::spawn(move || { // 使用 `move` 强制获取所有权
    //    println!("{:?}", person);
    // });
    // handle.join().unwrap();

    // thread::scope(|scope| { // 作用域线程
    //     scope.spawn(||{
    //         println!("person is {:?}", person);
    //     });
    // });

    // TODO： 案例
    // let total = 1000000000u64;
    // let mut sum = 0u64;
    // let  startTime = Instant::now();
    // for i in 0..  total {
    //     sum += i;
    // }
    // println!("总耗时 is {} ms", startTime.elapsed().as_millis());
    // println!("sum is {}", sum);
    count_time(serialized_calculate);
    count_time(parallel_calculate);
    count_time(scope_calculate);
    count_time(channal_calculate);
}

#[derive(Debug)]
struct Person {
    name: String,
}

fn count_time<F: FnOnce()>(f: F) {
    let name = any::type_name::<F>();
    let start_time = Instant::now();
    f();
    println!("{} 总耗时 is {} ms", name, start_time.elapsed().as_millis());
}

fn serialized_calculate() {
    let mut sum = 0u64;
    let startTime = Instant::now();
    for i in 0..TOTAL {
        sum += i;
    }
}

fn parallel_calculate() {
    let chunk_size = TOTAL / 16;
    let sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..16 {
        let start = i * chunk_size;
        let end = if i == 15 { TOTAL } else { (i + 1) * chunk_size };
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            for i in start..end {
                local_sum += i;
            }
            let mut sum = sum_clone.lock().unwrap();
            *sum += local_sum;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("sum is {}", sum.lock().unwrap());
}
// 局部线程
fn scope_calculate() {
    let chunk_size = TOTAL / 16;
    let sum = Mutex::new(0);
    thread::scope(|scope| {
        for i in 0..16 {
            let start = i * chunk_size;
            let end = if i == 15 { TOTAL } else { (i + 1) * chunk_size };

            let sum_ref = &sum;
            scope.spawn(move || {
                let mut local_sum = 0;
                for i in start..end {
                    local_sum += i;
                }
                let mut sum = sum_ref.lock().unwrap();
                *sum += local_sum;
            });
        }
    });
    println!("sum is {}", sum.lock().unwrap());
}

// 通道的概念
fn channal_calculate() {
    // mpsc::channel()
    let chunk_size = TOTAL / 16;
    let (tx, rx) = mpsc::channel();
    for i in 0..16 {
        let tx = tx.clone();
        let start = i * chunk_size;
        let end = if i == 15 { TOTAL } else { (i + 1) * chunk_size };
        thread::spawn(move || {
            let mut local_sum = 0;
            for i in start..end {
                local_sum += i;
            }
            tx.send(local_sum).unwrap();
        });
    }
    drop(tx);
    let mut sum = 0;
    while  let Ok(local_sum) = rx.recv() {
        sum += local_sum;
    }
    println!("sum is {}", sum);
}
