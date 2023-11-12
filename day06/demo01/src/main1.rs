use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;
// fn main() {
//     // TODO : Result 枚举与可恢复的错误

//     let result = read_username_from_file();
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     // let mut f = File::open("Hello.txt")?;
//     // let mut f = match f {
//     //     Ok(file) => file,
//     //     Err(e) => return Err(e),
//     // };

//     // TODO : ? 运算符 => 传播错误的一种快捷方式
//     File::open("Hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
//     // match f.read_to_string(&mut s) {
//     //     Ok(_) => Ok(s),
//     //     Err(e) => return Err(e),
//     // }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("Hello.txt")?;
    Ok(())
}
