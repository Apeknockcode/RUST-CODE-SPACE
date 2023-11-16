fn main() {
    // TODO : 迭代器
    /*
     * 迭代器模式: 对一系列项执行某些人物
     * 迭代器负责:
     *  - 遍历每个项
     *  - 确定序列(遍历)何时完成
     * Rust的迭代器:
     *  - 懒惰的: 除非调用消费迭代器的方法,否则迭代器本身没有任何效果.
     * */

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got : {}", val);
    }


    // TODO : Iterator trait 和 next 方法
    /*
     * TODO : Iterator trait 
     * 所有的迭代器都实现了 Iterator trait 
     * Iterator trait  定义于标准库,定义大致如下:
     * pub trait Iterator{
     *  type Item;
     *  fn next(&mut self) ->Option<Self::Item>;
     *  // 
     * }
     * Type Item 和 Self::Item 定义了于此该trait关联的类型
     *   - 实现了 Iterator trait 需要你定义一个Item类型,它用于next 方法的返回类型(迭代器的返回类型)
     * 
     * 
     * TODO : Iterator trait 
     * Iterator trait  仅要求实现一个方法: next 
     * 
     * next :
     *   - 每次返回迭代器中的一项
     *   - 返回结果包裹在Some里
     *   - 迭代器结束.返回None
     * 可以直接在迭代器上调用next方法
     * 
     * */ 

    //  TODO : 几个迭代方法
    /*
     * iter方法 : 在不可变引用上创建迭代器
     * into_iter 方法:创建的迭代器会获取所有权 
     * iter_mut 方法: 迭代可变的引用
     * 
     * */ 


    //  TODO : 消耗迭代器的方法
    /*
     * 在标准库中 Iterator trait 有一些带默认实现的方法
     * 其中有一些方法会调用next方法
     *   - 实现Iterator trait 时必须实现next 方法的原因之一
     * 调用next 的方法叫做 “消耗型适配器” 
     *   - 因为调用它们会吧迭代器消耗尽
     * 例如: sum 方法(就会消耗迭代器)
     *   - 取得迭代器的所有权
     *   - 通过反复调用next ,遍历所有元素
     *   - 每次迭代,把当前元素添加到一个总和里,迭代结束,返回总和 
     * */ 


    //  TODO : 产生其他迭代器的方法
    /*
     * 定义在 Iterator Trait 上的另外一些方法叫做 “迭代器适配器” 
     *  - 把当前的迭代器转换为不同类型的迭代器
     * 可以通过链式调用使用多个迭代器适配器来执行复杂的操作,这种调用可读性较高.
     * 例如: map.
     *  - 接受一个闭包,闭包作用于每个元素
     *  - 产生一个新的迭代器
     * */ 



    //  TODO :使用闭包捕获环境
    /*
     * fiter 方法:
     *  - 接受一个闭包
     *  - 这个闭包在遍历迭代器的每个元素时,返回bool类型
     *  - 如果闭包返回true: 当前元素将会包含在filter 产生的迭代器中
     *  - 如果闭包返回false: 当前元素将不会包含在filter产生的迭代器中 
     * 
     * */ 


    //  TODO : 使用Iterator trait 来创建自定义迭代器
    // 实现 next 方法





}



