// 枚举定义中的泛型

struct Point<T,U>{
    x:T,
    y:U
}

#[derive(Debug)]
enum  Option<T> {
    Some(T),
    None,
}

// 枚举也可以拥有多个泛型类型
enum Result<T,E>{
    OK(T),
    Err(E)
}

impl <T,U> Point<T,U>{
    fn x(&self) -> &T{
        &self.x
    }
}


// 方法定义中的泛型
fn main() {
    let p = Point{x:1,y:2.1};

    println!("p.x = {}", p.x());
}


