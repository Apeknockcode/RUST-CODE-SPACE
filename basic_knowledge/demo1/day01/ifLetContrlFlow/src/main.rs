#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
fn main() {
    println!("if let 简单控制流");
    // if let 语法让我们以一种不那么冗长的方式结合 if 和 let,
    // 来处理只匹配一个模式的值而忽略其他模式的情况

    let some_u8_value = Option::Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    // 我们可以使用 if let 这种更短方式编写.与match 行为一致
    if let Option::Some(4) = some_u8_value {
        println!("three")
    }
    /*
     * if let 获取通过等号分隔的一个模式和一个表达式。
     * 它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。
     * 换句话说，可以认为 if let 是 match 的一个语法糖，
     * 它当值匹配某一模式时执行代码而忽略所有其他值。
     */

    let mut count = 0;
    let state = UsState::Alabama;
    // let coin = Coin::Quarter(state);
    let coin = Coin::Dime;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // 使用这样的 if let 和 else 表达式
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("{}", count);
        return count += 1;
    }

   
}
