#[derive(Debug)]
enum MyEnum{
    Foo,
    Bar,
}


fn main(){
    // let v = Some(3);
    // match v {
    //     Some(3) =>{ println!("three" )},
    //     _ =>() 
    // }

    //  可不可以这么理解 ”let Some(3) = v“ 这是一个 if 条件
    // if let Some(3) = v {
    //     println!("Three");
    // }

    //  matchers! 宏
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    // println!("v的值{:?}",v);
    // v.iter().filter(|x| {matches!(x, MyEnum::Bar)});
    // println!("{:?}",v);
    // 变量遮蔽

//     let age = Some(1);
//     println!("在匹配前，age是{:?}",age);
//     if let Some(age) = age {
//         println!("匹配出来的age是{}",age);
//     }
//     println!("在匹配后，age是{:?}",age);

//     let age1 = Some(30);
//     println!("在匹配前，age是{:?}",age1);
//     match age1 {
//         Some(age1) =>  println!("匹配出来的age是{}",age1),
//         _ => ()
//     }
//     println!("在匹配后，age是{:?}",age1);

//     let age2 = Some(30);
//    println!("在匹配前，age是{:?}", age2);
//    match age2 {
//        Some(x) =>  println!("匹配出来的age是{}", x),
//        _ => ()
//    }
//    println!("在匹配后，age是{:?}", age2);


// let age = Some(30);
// let age_i = 30;
// if let Some(age) = age_i {

// println!("{}", age);
// }

// let age = Some(30);
// match age {
//    None => (),
//    Some(i) => {
//       println!("three {:?}", i);
//    }
// }

let a = 1;

    if let c = a {
        println!("c: {}", c);
    }

    let x: Option<(String, String)> = Some((String::from("Hello"), String::from("Rust")));
    if let Some((x, y)) = x {
        println!("{} {}", x, y);
    }

    let age = Some(18);
if let Some(age_) = age {
    println!("这里永远成立{:?}",age);
   println!("这里永远成立{:?}",age_);
}


}