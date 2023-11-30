fn main() {
    // println!("Hello, world!");
    // TODO : 控制测试如何运行
    /*
     * 改变cargo test 的行为: 添加命令行参数
     * 默认行为:
     *  - 并行运行
     *  - 所有参数
     *  - 捕获(不显示)所有输出,使读取与测试结果相关的输出更容易.
     *
     * 命令行参数:
     *   - 针对cargo test 的参数 , 紧跟cargo test 后
     *   - 针对 测试可执行程序: 放在 -- 之后
     * cargo test --help
     * cargo test -- --help
     * */
    // TODO : 并行运行测试
    /* 
     *  运行多个测试: 默认使用多个线程并行运行.
     *  - 运行快
     * 确保测试之间:
     *  - 不会相互依赖
     *  - 不依赖于某个共享状态(环境, 工作目录,环境变量等等)
     * */ 

    //  TODO : --test-threads 参数
    /* 
     *  传递给二进制文件
     *  不想以并行方式运行测试,或者想对线程进行细粒度控制
     *  可以使用 --test-threads 参数,后面跟着线程的数量
     * 例如: cargo test -- --test-threads=1
     * */ 

    //  TODO : 显示函数输出
    /*
     * 默认,如果测试通过 ,Rust 的test 库会捕获所有打印到标准输出的内容
     *  例如: 如果被测试代码中用到 printIn!: 
     *  - 如果测试通过,不会在终端看到printIn! 打印的内容.
     *  - 如果测试失败,会看到 printIn!打印的内容和失败的内容.
     * 如果想在成功的测试中看到打印的内容: 则 输入: cargo test -- --show-output
     * */ 

    //  TODO : 按名称运行测试的子集
    /*
     * 选择运行的测试: 将测试的名称(一个 或者 多个 ) 作为cargo test 的参数
     * 运行单个测试: 指定测试名
     * 例如 : cargo  test [指定测试的名称] 
     * 运行多个测试: 指定测试名的一部分(模块名也可以)
     * 例如: cargo test [指定测试函数名包含的相同的字母]
     * */ 

    //  TODO : 忽略某些测试,运行剩余的测试
    /*
     * ignore属性 (attribute) 
     * 在#[test] 下面标记 #[ignore] 对该测试函数进行忽略
     * 如何运行标记忽略的测试函数
     * 命令行: cargo test -- --ignored
     * */ 


    // TODO : 测试的分类
    /*
     * Rust 对测试的分类:
     *  - 单元测试
     *  - 集成测试
     * 单元测试:
     *  - 小,专注
     *  - 一次对一个模块进行隔离的测试
     *  - 可测试private接口
     * 集成测试:
     *   - 在库外部,和其他外部代码一样使用你的代码.
     *   - 只能使用 public 借口
     *   - 可能在每一个测试中使用多个模块 
     * */ 

    // TODO : #[cfg(test)] 标注
    /* 
     *  tests 模块上的#[cfg[test]]标注:
     *   - 只有运行cargo test 才编译和运行代码
     *   - cargo build 则不会
     * 集成测试在不同的目录,它不需要#[cfg(test)]标注
     * cfg: configuration(配置)
     *  - 告诉 Rust 下面的条目只能在指定的配置选项下才能被包含
     *  - 配置选项test: 由Rust提供,用来编译和运行测试
     *    - 只有cargo test 才会编译代码,包括模块中的helper函数 和 #[test] 标注的函数
     * */ 

    //  TODO : 测试私有函数

    // TODO : 集成测试
    /*
     * 在Rust 里.集成测试完全位于被测试库的外部
     * 目的: 是测试被测试库的多个部分是否能正常的一起工作
     * 集成测试的覆盖率很重要
     * */ 

    //  TODO : Tests 目录
    /* 
     * 命令行: cargo new adder --lib 创建测试项目
     * 
     * 创建集成测试: tests目录 
     * tests 目录下的每个测试文件都是单独的一个crate
     * 无需标注#[cfg(test)] , tests 目录被特殊对待
     *   - 只有cargo test,才会编译tests目录下的文件
     * 
     * */ 


    //  TODO : 运行指定的集成测试
    /*
     *  运行一个特定的集成测试 : cargo test 函数名
     *  运行某个测试文件内的所有测试 : cargo test --test 文件名
     * 
     * */ 


    //  TODO : 集成测试中的子模块(子文件)
    /*
     * tests 目录下每个文件被编译称单独的crate
     *  - 这些文件不共享行为(与src下的文件规则不同)
     * */ 


    //  TODO : 针对binary crate 的集成测试
    /*
     * 如果项目是 binary crate ,只含有src/main.rs 没有src/lib.rs
     *  - 不能在tests 目录下创建集成测试
     *  - 无法把main.rs的函数导入作用域
     * 只有 library crate 才能暴露函数给其他crate 用 
     * binary crate 意味着独立运行
     * 
     * 
     * */ 


}
