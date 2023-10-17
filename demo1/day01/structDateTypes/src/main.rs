fn main() {
    println!("定义并实例化结构体");
    /*
     * 结构体和我们在“元组类型”章节论过的元组类似。
     * 1.结构体的每一部分可以是不同类型
     * 2.结构体需要命名各部分数据以便能清楚的表明其值的意义
     * 定义结构体:
     * 1.需要使用 struct 关键字并为整个结构体提供一个名字
     * */

    //  使用User，通过为每个字段指定具体值来创建这个结构体的实例
    // 创建 User 结构体的实例
    let mut user1 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@email.com"),
        sign_in_count: 1,
    };

    // 从结构体中获取某个特定的值，可以使用点号
    println!("user1: {}", user1.username);
    // 更改结构体中的值
    user1.email = String::from("someone@email.com");

    let mut user2 = build_user("AAAAA".to_string(), "EMAIL".to_string());
    println!("user2:name: {:?},{:?}", user2.username, user2.email);

    // 使用结构体 更新语法从其他实例创建
    // 使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常很有用.这可以通过结构体更新语法（struct update syntax）实现

    // ------- 不使用更新语法时
    // let user3 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // ------- 使用结构体更新语法
    let user4 = User {
        email: String::from("another@example.com"),
       ..user1
    };

}

// 示例
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    return User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    };
}
