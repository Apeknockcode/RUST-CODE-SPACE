use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");
    // 哈希 map 储存键值对
    /*
     * 常用集合类型是 哈希 map（hash map）
     * HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射
     * 哈希 map 可以用于需要任何类型作为键来寻找数据的情况
     * 像 vector 一样，哈希 map 将它们的数据储存在堆上，
     * 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。
     *  **/

    //  新建一个哈希 map

    // 新建一个哈希 map 并插入一些键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // 另一种方法:使用一个元组的 vector 的 collect 方法
    // collect 方法可以将数据收集进一系列的集合类型，包括 HashMap。
    // 使用 zip 方法来创建一个元组的 vector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 40];

    let scores_1: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores_1);

    // 哈希 map 和所有权

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 当 insert 调用将 field_name 和 field_value 移动到哈希 map 中后，将不能使用这两个绑定。
    // println!("field_name is {}", field_name); // 这里 field_name 和 field_value 不再有效，

    /*
     * 访问哈希 map 中的值
     * 可以通过 get 方法并提供对应的键来从哈希 map 中获取值
     *  **/

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);
    println!("score is {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新哈希 map
    scores.insert(String::from("Blue"), 25);
    println!("scores is {:?}", scores);

    // 只在键没有对应值时插入
    scores.entry(String::from("RED")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("scores is {:?}", scores);

    // 根据旧值更新一个值

    let text = "hello world wonderful world";

    let mut map1 = HashMap::new();
    //  split_whitespace() 这是获取字符串的空格字符

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map1);

}
