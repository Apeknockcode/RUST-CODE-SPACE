### 创建文件 main.rs
```rust
fn main(){
    println("Hello world");
}
```
### 运行文件
```bash
rusts main.rs
./main
Hello world
```

### 通过 cargo 创建项目
```bash
cargo new [项目名字]
cd [项目名称]
```
### 运行项目
```bash
cargo run
```
### 打包项目
```bash
cargo build
```
### 检查代码确保其可以编译
```bash
cargo check
```

我们回顾下已学习的 Cargo 内容：

可以使用 cargo build 构建项目。<br/>
可以使用 cargo run 一步构建并运行项目。<br/>
可以使用 cargo check 构建项目而无需生成二进制文件来检查错误。<br/>
有别于将构建结果放在与源码相同的目录，Cargo 会将其放到 target/debug 目录。<br/>
