// 结构体定义中的泛型

// struct Point<T> {
//     x: T,
//     y: T,
// }


struct Point<T,U>{
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 4, y: 4 };
    let float = Point { x: 1.3, y: 1.3 };

    //  字段 x 和 y 的类型必须相同，因为他们都有相同的泛型类型 T
    let float = Point { x: 1.3, y: 1 };
}
