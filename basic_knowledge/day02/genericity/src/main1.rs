// fn main() {
//     // println!("Hello, world!");
//     // 泛型数据类型
//     // 如何使用泛型定义函数、结构体、枚举和方法，然后我们将讨论泛型如何影响代码性能。

//     // 在函数定义中使用泛型
//     // 展示了两个函数，它们的功能都是寻找 slice 中最大值
// }

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}
use std::cmp::PartialOrd;
// 使用泛型归类函数
fn largest<T: PartialOrd>(list: &mut [T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

// 在函数定义中使用泛型
fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    let mut number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&mut number_list);
    println!("The largest number is {}", result);

    let mut char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&mut char_list);
    println!("The largest char is {}", result);
}
