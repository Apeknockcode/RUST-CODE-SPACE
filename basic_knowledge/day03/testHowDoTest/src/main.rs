fn main() {
    //TODO : 控制测试如何运行
    /*
     * 运行 cargo test --help 会提示 cargo test 的有关参数
     *
     * **/

    //  TODO : 并行或连续的运行测试
    /*
     * 运行多个测试时， Rust 默认使用线程来并行运行
     * 因为测试是在同时运行的，你应该确保测试不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境.
     * 一个解决方案是使每一个测试读写不同的文件；另一个解决方案是一次运行一个测试。
     *
     * 如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，
     * 可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件
     * $ cargo test -- --test-threads=1
     * 这里将测试线程设置为 1，告诉程序不要使用任何并行机制
     *
     * */

    //  TODO : 显示函数输出
    /*
     * 默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容
     * 如果你希望也能看到通过的测试中打印的值，截获输出的行为可以通过 --nocapture 参数来禁用：
     * $ cargo test -- --nocapture
     *
     * */

    //  TODO : 通过指定名字来运行部分测试
    /*
     * 你可以向 cargo test 传递所希望运行的测试名称的参数来选择运行哪些测试。
     * **/

    //  TODO : 运行单个测试
    /*
     *  可以向 cargo test 传递任意测试的名称来只运行这个测试：
     *  cargo test [方法名]
     * **/

    //  TODO : 过滤运行多个测试
    /*
     *  可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行
     *  cargo test [测试方法关键字]
     * */

    //  TODO : 忽略某些测试
    /*
     * 虽然可以通过参数列举出所有希望运行的测试来做到，也可以使用 ignore 属性来标记耗时的测试并排除他们
     * #[test]
     * #[ignore]
     * fn expensive_test() {
     *           ....
     * }
     * */
}
