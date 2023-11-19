// TODO : Rc<T> 引用计数智能指针
/*
 * 有时, 一个值会有多个所有者
 * 为了支持多重所有权:Rc<T>
 *   - reference couting (引用计数)
 *   - 追踪所有到值的引用
 *   - 0个引用: 该值可以被清理掉
 * 需要在 heap 堆内存 上分配数据,这些数据被程序的多个部分读取(只读),但在
 * 编译时,无法确定哪个部分最后使用完这些数据
 *
 * TODO : Rc<T> 智能用于单线程场景
 *
 * */

//  TODO : 例子 Rc<T> 不在预导入模块中
/*
 * Rc<T> 不在预导入模块中
 * Rc::clone(&a) 函数: 会增加引用计数 ,不会进行深度克隆
 * Rc::Strong_count(&a) : 会获的(强)引用计数
 *   - 还有Rc::weak_count 函数 : 弱引用计数
 * */

//  例子: 两个list 共享 另一个list的所有权
/*
 *  b -> 3 -
 *          |
 *      a ->+ -> 5 —> 10 -> Nil
 *          |
 *  c -> 4 -
 * */

// enum List{
//     Cons(i32,Box<List>),
//     Nil
// }

// use crate::List::{Cons,Nil};

//  fn main() {
//     let a =Cons(5,Box::new(Cons(10,Box::new(Nil))));
//     let b = Cons(3,Box::new(a));
//     let c =Cons(4,Box::new(a));
//  }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a ={}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));

//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after creating  scope= {}", Rc::strong_count(&a));
// }


/*
 * Rc::clone()  vs 类型的clone() 方法
 *   - Rc::clone() : 增加引用计数,不会执行数据的深度拷贝操作
 *   - 类型的 clone() : 很多会执行数据的深度拷贝操作
 * Rc<T> : 通常不可变引用,使你可以在程序不同部分之间共享只读数据
 * 
 * */ 


//  TODO : 内部可变性
/*
 * 内部可变性是rust 的设计模式之一
 * 它允许你在只持有不可变引用的前提下对数据进行修改
 *  - 数据结构中数用了 unsafe 代码来绕过Rust 正常的可变性和借用规则
 * RefCell<T> 与Rc<T> 不同,RefCell<T> 类型代表了其持有数据的唯一所有权
 * 回忆一下借用规则:
 *   -在任何给定的时间里,你要么只能拥有一个可变引用,要么只能拥有任意数据的不可变引用.引用总数有效的.
 * 
 * TODO :  RefCell<T> 与 Box<T> 的区别
 *    Box<T>                            RefCell<T>
 *     - 编译阶段强制代码遵守借用规则          - 只会在运行时检查借用规则
 *     - 否则出现错误                       - 否则会出发panic!
 * 
 * TODO : 借用规则 :在不同阶段进行检查的比较
 * 编译阶段                         运行时
 *  - 尽早暴露问题                    - 问题暴露延后,甚至到生产环境
 *  - 没有任何运行时开销               - 因借用计数产生些许性损失
 *  - 对大多数场景是最佳选择            - 实现某些特定的内存安全场景(不可变环境中修改自身数据) 
 *  - 是Rust 的默认行为
 * 
 * ReCell<T> 与 Rc<T> 相似,只能用于单线程场景
 * 
 * 选择Box<T> Rc<T> RefCell<T>的依据
 *                    Box<T>                 Rc<T>               RefCell<T>
 * 同一数据的所有者     一个                     多个                 一个
 * 可变性,借用检查   可变,不可变借用(编译时检查) 不可变借用(编译时检查)   可变,不可变借用(运行时检查) 
 * 
 * 其中: 即便RefCell<T> 本身不可变,但仍能修改其中存储的值
 * 
 * 内部可变性: 可变的借用一个不可变的值.
 * 
 * 
 * */ 

fn main(){
    let x = 5;
    // 推论: 无法可变的借用一个不可变的值
    //  y 借用了 一个无法可变的x 的值
    // let y = &mut x; // 报错
}


