
// TODO: 泛型
// fn main(){
//     let result = add(1,2);
//     println!("result = {}", result);
//     let point1 = Point { x: 1, y: 2 };
//     let point2 = Point { x: 2, y: 3 };
//     let result = add(point1,point2);
//     println!("result is {:#?} ", result)
// }
// fn add<T:Add<Output = T >>(x:T,y:T)->T{
//     x + y
// }
// impl Add for Point {
//     type Output = Self;
//     fn add(self, other: Self) -> Self::Output {
//         Point { x: self.x + other.x, y: self.y + other.y }
//     }
// }
// #[derive(Debug)]
// struct Point<T = i32>{
//     x:T,
//     y:T
// }
//



// use std::fmt::format;
// use std::io;
// use std::io::Error;

/**----------------------------------------------------*/
// TODO： 闭包
/**
语法： Let add_one = ｜x｜ x + 1
捕获环境： 借用，可变借用，获取所有权
特征： Fn FnMut FnOnce
*/
// fn main() {
//     let sum = |a: i32, b: i32| a + b; // 闭包定义
//     println!("{}", sum(1, 2));
//     struct MyFun{
//         name: String
//     }
//     impl  MyFun{
//         fn new(name: String) -> Self {
//             Self {name}
//         }
//         fn call(self){
//             say_Hello(self.name);
//         }
//     }
//     let name = String::from("Tyr");
//     // let say = || say_Hello(name);
//     let say = MyFun::new(name);
//     say.call();
// }

// fn say_Hello(name:String){
//     println!("Hello , {}", name);
// }

// fn delegate<F:Fn()>(f:F){
//     f();
//     f();
// }


// TODO：错误处理
/**
painc！： 不可恢复错误
Result<T,E>:可恢复错误
处理方法： match。unwarp() expect() 运算符
**/


// fn  main()-> Result<(),MyError> {
//     let age = 20;
//     // let result = do_something(age).expect("too young");
//    // println!("Result is {}", result);
//    //  let result = do_something(age);
//     // match result {
//     //     Ok(_) => println!("Success"),
//     //     Err(error) => println!("Error: {}", error)
//     // }
//     let result = do_something(age)?;
//     println!("The result is {}", result);
//     Ok(())
// }
// fn do_something(age: u32) -> Result<String, i32> {
//     if age > 20{
//         Ok(String::from("大于20岁"))
//     }else{
//         Err(10000)
//     }
// }
// #[derive(Debug)]
// struct MyError(String);
//
// impl  From<i32> for MyError {
//     fn from(code: i32) -> Self {
//         match code{
//             1=>MyError("Biz error".to_string()),
//             10000=>MyError("to young_error".to_string()),
//             _ => MyError("Unknown error".to_string())
//         }
//     }
// }
// enum  AppError{
//     DatabaseError(String),
//     JWTError(String),
//     Unauthorized(String),
//     BadRequest(i32),
// }
// impl From<io::Error> for AppError {
//     fn from(error:Error) -> Self {
//         Self::IoError(format!("io error: {}", error))
//     }
// }


// 模块和库
/**
mod
crate
测试
*/

// fn main(){
//
// }
// 定义模块
// crate - 当前包的根模块
// 作用：pub(crate) 表示该项在当前 整个包（crate） 内可见，但对包外部是私有的。
// 用途：适用于需要跨模块共享但不想暴露给外部的项（如内部工具函数或类型）。
// super - 父模块
// 作用：pub(super) 表示该项仅对 父模块 可见，其他地方（包括同包其他模块）不可见。
// 用途：限制可见性到直接父模块，适用于模块内部的分层设计。
// mod parent {
//     #[derive(Debug)]
//     pub struct PubStruct;// 定义公有结构体
//     #[derive(Debug)]
//     struct  PrivateStruct;// 定义私有结构体
//     #[derive(Debug)]
//     struct Demo{
//         sub_pub: child::PubSubStruct,
//         sub_pri: child::PrivateSubStruct,
//     }
//
//
//     // 定义子模块
//     mod child{
//         use crate::parent::{PrivateStruct, PubStruct}; // 绝对路径
//
//         #[derive(Debug)]
//         pub struct PubSubStruct{
//             pub_struct: PubStruct,
//             pri_struct: PrivateStruct
//         }
//         #[derive(Debug)]
//         pub(super) struct PrivateSubStruct;
//
//     }

// #[cfg(test)]
// mod tests{
//
// }



// 模块引用
mod math;
use math::add;
fn main(){
    let result = add(1,2);
    println!("{}", result);
}


