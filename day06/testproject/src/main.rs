fn main() {
    // println!("Hello, world!");
    // TODO : 测试(函数)
    /*
     * 测试: 
     *  - 函数
     *  - 验证非测试代码的功能是否和预期一致
     * 测试函数体(通常) 执行的3个操作:
     *  - 准备数据/状态
     *  - 运行被测试的代码
     *  - 断言(Assert)结果
     * */ 


    //  TODO : 剖析测试函数
    /* 
     *  测试函数需要使用test(属性) attribute 进行标注
     *   - Attribute 就是一段Rust代码的元数据
     *   - 在函数上加#[test] , 可把函数变成测试函数
     *  运行测试:
     *   - 使用cargo test 命令运行所有测试函数
     *    - Rust 会构建一个Test Runner 可执行文件 
     *      - 它会运行标注了test 的函数,并报告其运行是否成功
     *  当使用cargo 创建library 项目的时候,会生成一个test module,里面有一个 test 函数
     *   - 你可以添加任意数量的test module 或函数
     * 
     * 测试失败
     *  - 测试函数 panic 就表示失败
     *  - 每个测试运行在一个新线程.
     *  - 当主线程看见某个测试线程挂掉了,那个测试标记为失败
     * */ 


    //  TODO : 使用断言(assert! 宏)检查测试结果
    /*
     * assert! 宏. 来自标注库,用来确定某个状态是否为 true
     *  - true: 测试通过
     *  - false: 调用 panic! 测试失败
     * 
     * */ 

    //  TODO : 使用 asset_eq! 和 assert_ne! 测试相等性
    /*
     * 都来自标准库 
     * 判断两个参数是否相等 或 不等
     * 实际上,他们使用的就是 == 和 != 运算符
     * 断言失败: 自动打印出两个参数的值
     *   - 使用debug格式打印参数
     *    - 要求参数实现了 PartialEq 和 Debug Trait (所有的基本类型和标准库里大部分类型都实现了)
     * */ 



    //  TODO : 添加自定义报错信息
    /*
     *   可以向 assert! , assert_eq! , assert_ne! 添加可选的自定义消息
     *    - 这些自定义消息和失败消息都会打印出俩
     *    - assert! 第一参数必填,自定义消息作为第2个参数
     *    - assert_eq! 和 assert_ne! : 前两个参数必填,自定义消息作为第三个参数
     *    - 自定义消息参数会被传递给 format! 宏 ,可以使用 {} 占位符
     * */ 


    // TODO :  验证错误处理的情况
    /*
     * 测试除了验证代码的返回值是否正确,还需验证代码是否如预期的处理发生错误的情况
     * 可验证代码在特定情况下是否发生了panic!
     * should_panic 属性(attribute) :
     *  - 函数 panic: 测试通过
     *  - 函数没有 panic! : 测试失败
     * 让should_panic 更精确
     *  为should_panic 属性添加一个可选的expected参数
     *  - 将检查失败消息中是否包含所指定的文字
     * */ 



    //  TODO : 在测试中使用 Result<T,E >
    /*
     * 无需 panic,可使用 Result<T,E> 作为返回类型编写测试:
     *  - 返回 OK : 测试通过 
     *  - 返回 Err : 测试失败
     * 注意: 不要在使用 Result<T , E> 编写的测试上标注 #[should_panic]  
     * */ 


    //  






}
