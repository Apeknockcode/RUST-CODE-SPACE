// TODO : send 和 Sync trait 
/*
 * Rust 语言的并发特性较少,目前讲的并发特性都来自于标准库(而不是语言本身)
 * 无需局限于标准库的并发,可以自己实现并发
 * 但在Rust 语言中有两个并发的概念
 *  - std::marker::Sync 和 std::marker::Send 这两个trait
 * TODO: Send : 允许线程间转移所有权 
 * 实现Send trait 的类型可在线程间转移所有权
 * Rust 中几乎所有的类型都实现了Send 
 *   - 但是 Rc<T> 没有实现send ,它只用于单线程情景
 * 任何完全有Send 类型组成的类型也被标记为 Send 
 * 除了原始指针之外,几乎所有的基础类型都是实现了Send 
 * TODO: Sync : 允许从多个线程访问
 * 实现Sync的类型可以安全的被多个线程引用
 * 也就是说: 如果T 是Sync,那么 &T 就是Send
 *  - 引用可以被安全的送往另一个线程
 * 基础类型都是Sync 
 * 完全有Sync类型组成的类型也是Sync 
 *  - 但是 Rc<T> 不是Sync的
 *  - RefCell<T> 和 Cell<T> 家族也不是Sync的
 *  - 而 Mutex<T> 是Sync的
 * TODO:手动来实现Send 和 Sync 是不安全的
 * 
 * */ 

 fn main(){
    println!("over")
 }