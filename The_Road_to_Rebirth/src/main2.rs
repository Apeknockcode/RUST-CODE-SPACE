use std::cell::{Ref, RefCell};
use std::rc::Rc;

fn main() {
//     println!("Hello, world!");
//     let mut typer = UserTyper::new(CommandLineComputer);
//     typer.type_expr();
//     print!("Result : {}",typer.computer());
//     let name = RefCell::new(String::from("Taotao"));
//     // println!("name is {}", name.borrow()) // 可变借用
//     name.borrow_mut().push_str("6666");
//     println!("name is {}", name.borrow());
    let name = Rc::new(String::from("TaoTao"));
    let user = User{
        name: Rc::clone(&name),
    };
    println!("user is {}", user.name);
    let employee  = Employee{
        name: Rc::clone(&name)
    };
    println!("employee is {}", employee.name);
    println!("user_debugger is {:#?},employee_debugger is {:#?}", user, employee);
}

// TODO: Rc指针:解决内存共享的问题
#[derive(Debug)]
struct  User{
    name: Rc<String>, // 引用技术 refencet count
}
#[derive(Debug)]
struct  Employee{
    name : Rc<String>,
}