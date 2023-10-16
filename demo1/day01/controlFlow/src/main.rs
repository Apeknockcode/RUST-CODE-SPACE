fn main() {
    println!("控制流 - 根据条件是否为真来决定执行某些代码");
    let number = 4;
    if number > 5 {
        println!("number 大于 5");
    } else if number == 4 {
        // 使用 else if 处理多重条件
        println!("number等于4");
    } else {
        println!("number小于5");
    }

    // 在 let 语句中使用 if
    let booleanValue = true;
    let otherNumber = if booleanValue { 4 } else { 6 };
    println!("otherNumber : {}", otherNumber);

    // 使用循环重复执行 - Rust 为此提供了多种循环（loop）
    // 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。
    // loop {
    //     println!("again");
    // }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remining  = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1
    }
    println!("end count = {}", count);

    // 从循环返回
    // loop的一个用例是重试可能会失败的操作.你可以在用于停止循环的 break 表达式添加你想要返回的值

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);


    // while 条件循环
    // 在程序中计算循环的条件也很常见。
    // 当条件为真，执行循环。当条件不再为真，调用 break 停止循环。
    // 这个循环类型可以通过组合 loop、if、else 和 break 来实现；
    
    let mut num = 4;
    while num !=0{
        println!("{}!", num);
        num -=1;
    }
    println!("LIFTOFF");

    // 使用for遍历集合
    // 可以使用 while 结构来遍历集合中的元素，比如数组。

    let array = [1,2,3,4,5];
    let mut index = 0;
    while index<5{
        println!("the value is :{}", array[index]);
        index +=1
    }
    
}
