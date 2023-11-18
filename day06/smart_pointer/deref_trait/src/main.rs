// TODO : Deref trait (解引用的意思)
/*
 * 实现Deref trait 使我们可以自定义接引用运算符 * 的行为
 * 通过实现Deref ,智能指针可以像常规引用一样来处理
 *
 * 解引用运算符
 *  常规引用是一种指针
 *
 * */

// fn main() {
//     let x = 5;
//     let y = &x;
//     assert_eq!(x, 5);
//     assert_eq!(*y, 5);
// }

// TODO : 把Box<T> 当作引用
/*
 * Box<T> 可以代替上例中的引用
 *
 * */

// fn main() {
//     let x = 5;
//     let y = Box::new(x);
//     assert_eq!(x, 5);
//     assert_eq!(*y, 5);
// }

// TODO : 定义自己的智能指针

/*
 * Box<T> 被定义成拥有一个元素的Tuple struct
 * (例子) MyBox<T>
 * TODO : 实现Deref Trait
 *      标准库中的Deref trait 要求我们实现一个deref方法“
 *    -- 该方法借用self
 *    -- 返回一个指向内部数据的引用
 * */

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//     assert_eq!(x, 5);
//     assert_eq!(*y, 5);
// }

// TODO : 函数和方法的隐式解引用转化( Deref Coercion)
/*
 * 隐式解引用转化是为函数和方法提供的一种便捷特性
 * 假设T 实现了 Deref trait :
 *   - Deref Coercion 可以把T的引用转化为 T 经过Deref 操作后生成的引用
 * 当把某类型的引用传递给函数或方法时,但他的类型与定义的参数类型不匹配;
 *   - Deref Coercion 就会自动发生
 *   - 编译器会对Deref 进行一系列调用,来把它转化为所需的参数类型
 *     - 在编译时完成,没有额外性能开销
 * */

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn hello_name(name: &str) {
//     println!("Hello, {}", name);
// }

// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello_name(&m);
//     hello_name("Rust");
// }

// TODO : 解引用与可变性
/*
 * 可使用 DerefMut trait 重载可变引用的* 运算符
 * 在类型和 trait 在下列三种情况发生时,Rust 会执行 deref coercion :
 *   - 当 T: Deref<Target = U> , 允许&T 转化为 &U;
 *   - 当 T: DerefMut<Target = U> , 允许&mut T 转化为 &mut U
 *   - 当 T: Deref<Target = U> , 允许&mut  T 转化为&U;
 * */

//  TODO : Drop trait

/*
 * 实现Drop trait , 可以让我们自定义当值将要离开作用域时发生的动作
 *   - 例如: 文件,网络资源释放等
 *   - 任何类型都可以实现Drop trait
 *
 * Drop trait 只要求你实现drop方法
 *   - 参数: 对self 的可变引用
 *
 * Drop trait 在预导入模块里(prelude)
 *
 * */
use std::mem::drop;
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("with data : {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("My stuff"),
    };
    // 提前清理 drop 
    drop(c);
    // c.drop();
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
}

//  TODO : 使用std::mem::drop 来提前drop 值
/*
 * 很难直接禁用自动的drop功能,也没必要
 *   - Drop trait 的目的就是进行自动的释放处理逻辑
 * Rust 不允许手动调用 Drop trait 的drop 方法
 *   - 但可以调用标准库的std::mem::drop 函数,来提前drop值 
 *
 * */
