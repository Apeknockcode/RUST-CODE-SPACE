use std::cmp::Ord;

fn main() {
    // TODO : 提取函数以及消除重复代码
    let number_list = vec![1, 2, 3, 4, 5, 6, 7];
    // let mut largest = number_list[0];
    // println!("{}", largest);

    // for item in number_list {
    //     println!("初始最大的值: {}", largest);
    //     println!("遍历: {}", item);
    //     if item > largest {
    //         largest = item
    //     }
    // }
    let result1 = get_largest_number(&number_list);
    println!("最大值 : {}", result1);

    let char = vec!["1","2","3","4","5","6","7"];

    let result2 = get_largest_number(&number_list);
    println!("最大值 : {}", result2);
}

// fn get_largest_number(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

fn get_largest_number<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
