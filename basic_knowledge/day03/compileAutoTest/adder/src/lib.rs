pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    // 使用 assert! 宏来检查结果
    /*
     * 向 assert! 宏提供一个求值为布尔值的参数。
     * 如果值是 true，assert! 什么也不做，同时测试会通过。
     * 如果值为 false，assert! 调用 panic! 宏，这会导致测试失败
     * **/

    #[test]
    fn larger_can_hold_samller() {
        let larger = Rectangle {
            width: 4,
            height: 2,
        };
        let samller = Rectangle {
            width: 2,
            height: 1,
        };

        assert!(larger.can_hold(&samller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger)); // 因为这里 can_hold 函数的正确结果是 false ，我们需要将这个结果取反后
    }

    //  使用 assert_eq! 和 assert_ne! 宏来测试相等
    /*
     * 将需要测试代码的值与期望值做比较，并检查是否相等。可以通过向 assert! 宏传递一个使用 == 运算符的表达式来做到
     * assert_eq! 和 assert_ne!。这两个宏分别比较两个值是相等还是不相等。
     * 当断言失败时他们也会打印出这两个值具体是什么，以便于观察测试 为什么 失败，
     * 而 assert! 只会打印出它从 == 表达式中得到了 false 值，而不是导致 false 的两个值
     * */

    #[test]
    fn it_add_two() {
        assert_eq!(6, add_two(4));
        /*
         * assert_eq! 的 left 参数是 6，而 right 参数，也就是 add_two(2) 的结果，是 7
         * 断言两个值相等的函数的参数叫做 expected 和 actual，而且指定参数的顺序是很关键的。
         * 然而在 Rust 中，他们则叫做 left 和 right，同时指定期望的值和被测试代码产生的值的顺序并不重要
         * */

        // assert_ne! 宏在传递给它的两个值不相等时通过，而在相等时失败。

        // 总之:assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=
    }

    // 自定义失败信息
    // 你也可以向 assert!、assert_eq! 和 assert_ne! 宏传递一个可选的失败信息参数，可以在测试失败时将自定义失败信息一同打印出来

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));
        // 自定义失败信息
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // 使用 should_panic 检查 panic
    //  除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误也是很重要的
    // #[should_panic]
    // 移除 new 函数在值大于 100 时会 panic 的条件
    // should_panic 甚至在一些不是我们期望的原因而导致 panic 时也会通过
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // 为了使 should_panic 测试结果更精确，我们可以给 should_panic 属性增加一个可选的 expected 参数
    fn greater_than_100() {
        Guess::new(200);
    }


    // 将 Result<T, E> 用于测试
    // TODO : 我们编写的测试在失败时就会 panic。也可以使用 Result<T, E> 

    #[test]
    fn it_works_result() -> Result<(),String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello!")
}
