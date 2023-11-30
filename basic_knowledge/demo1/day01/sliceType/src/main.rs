fn main() {
    // 另一个没有所有权的数据类型是 slice
    // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
    let mut s = String::from("hello world");
    // let worlds = first_word(&s);
    // println!("world is {}",worlds );
    // s.clear();// 这清空了字符串，使其等于 ""

    // 字符串slice
    let hello = &s[0..5];
    println!("hello is {}", hello);
    let world = &s[6..11];
    println!("world is {}", world);
    let len = s.len();
    let hello_slice = &s[0..len];
    println!("hello_slice is {}", hello_slice);
    let hello_slice = &s[0..];
    println!("hello_slice is {}", hello_slice);

    let word = re_first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);

    let a = [1, 2, 3, 4, 5];

    let slices = &a[1..3];
    println!("slices is {:?}", slices);

}

fn re_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("re_first_word is {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
