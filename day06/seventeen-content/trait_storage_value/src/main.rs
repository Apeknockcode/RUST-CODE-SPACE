// TODO 为共有行为定义一个trait

/*
 * Rust 避免讲struct 或enum 称为对象,因为他们与impl 块是分开的
 *
 * trait 对象有些类似于其他语言中的对象
 *   - 他们某种程度上组合了数据与行为
 * trait 对象与传统对象不同的地方:
 *   - 无法为trait 对象添加数据
 * trait 对象被专门用于抽象某些共有行为.它没其他语言中的对象那么通用
 * */

use seventeenContent::trait_storage_value::Draw;
use seventeenContent::trait_storage_value::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制一个选择框
    }
}
fn main() {}


// TODO. : 
