// TODO : 状态模式
/*
 * 状态模式 是一种面向对象设计模式 :
 *   - 一个值拥有的内部状态由数个状态对象表达而成,而值的行为则随着内部状态的改变而改变
 * 使用状态模式意味着:
 *   - 业务需求变化时,不需要修改持有状态的值的代码,或者使用这个值的代码
 *   - 只需要更新状态内部的代码,以便改变其规则,或者增加一些新的状态对象
 * 
 * */

 use design_state_mode::Post;

 fn main(){
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today",post.content());
 } 