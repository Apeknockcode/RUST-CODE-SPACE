use std::net::IpAddr;
fn main() {
    // TODO : 何时使用 panic! 宏
    /*
     * 建议:
     * 在定义一个可能失败的函数时,优先考虑返回Result
     * 否则就 panic! 宏
     * 例子: 可以使用panic!
     *  - 演示某些概念: unwrap
     *  - 原型代码 unwrap,expect
     *  - 测试代码 unwrap,expect
     * 错误处理的指导性建议:
     *  - 当代码最终可能处于损坏状态时,最好使用panic!
     *  - 损坏状态(Bad state) : 某些假设,保证,约定或不可变性被打破
     * 场景建议:
     *  - 调用你的代码,传入无意义的参数值:panic!
     *  - 调用外部不可控代码,返回非法状态,你无法修复:panic!
     *  - 如果失败是可预期的:Result
     *  - 当你的代码对值进行操作,首先应该验证这些值;不符合则:panic!
     * 为了验证创建自定义类型
     *  - 创建新的类型,把验证逻辑放在构造实例的函数里.
     *  -
     * */

    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("ipAddr : {}", home);

    // TODO : 为了验证创建自定义类型
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);
    };
}

// TODO : 为了验证创建自定义类型
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100,get {}", value);
        }
        // 创建一个Guess 实例
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
