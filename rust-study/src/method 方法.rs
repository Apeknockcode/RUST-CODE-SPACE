// 方法Method
// 定义方法

#[derive(Debug)]
// 定义一个结构体
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
impl Circle {
    // 初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // 计算圆面积的方法
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 该矩形的面积
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    //  test 计算圆的面积
    let round = Circle {
        x: 10.0,
        y: 10.0,
        radius: 10.0,
    };

    // 计算面积
     println!("round is area {:?}", round.area());


     let rect = Rectangle{
        width:20,
        height:10,
     };
     let rect_area_result = rect.area();
     println!("rect_area_result is {:?}", rect_area_result);
}
