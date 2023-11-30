#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 感觉这里就跟js中原型链一样, 这个Rectangle结构体中,带有area方法
// 这个 impl 块中的所有内容都将与 Rectangle 类型相关联
impl Rectangle {
    fn area(&self) -> u32 {
        dbg!(&self);
        return self.width * self.height;
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 多个 impl 块
impl Rectangle {
    fn height(&self) -> bool {
        return self.height > 0;
    }
}

fn main() {
    println!("方法语法");
    /*
     * 方法 与函数类似：
     * 它们使用 fn 关键字和名称声明，可以拥有参数和返回值，
     * 同时包含在某处调用该方法时会执行的代码
     * */

    //  ----------------------------------------------------------------
    /*
     *  方法与函数是不同的，因为它们在结构体的上下文中被定义.
     *  并且它们第一个参数总是 self，它代表调用该方法的结构体实例
     * */

    //  定义方法

    let rect = Rectangle {
        width: 30,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width)
    }

    // 带有更多参数的方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 关联函数
}
