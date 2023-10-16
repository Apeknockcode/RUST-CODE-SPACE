// 入口函数
fn main() {
    println!("Hello, world!");
    another_function();
    another_function_1(4);
    print_labeled_measurement(4,'4');
    Statements_and_expressions(2);
    let value = ReturnValueFunction();
    println!("return value: {}", value);
    let value1= plus_one(5);
    println!("value1 is {}", value1);
}

fn another_function() {
    println!("Another function");
}

// 携带参数函数
fn another_function_1(x: i32) {
    println!("Another function——1:{}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is:{},{}", value, unit_label);
}


// 语句和表达式
fn Statements_and_expressions(x:i32){
    // 表达式
    let a= {
        let a =4;
        x+a
    };
    println!("a is:{}", a);
}


// 带有返回值的函数
fn ReturnValueFunction()->i32{
   return  5 // 5    
}
fn plus_one(x:i32)->i32{
    return x + 100
}

