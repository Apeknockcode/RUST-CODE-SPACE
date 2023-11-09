## 章节: 一个I/O : 构建一个命令行程序

> Rust 的运行速度、安全性、单二进制文件输出和跨平台支持使其成为创建命令行程序的绝佳选择，所以我们的项目将创建一个我们自己版本的经典命令行工具：grep.
>
> grep 最简单的使用场景是在特定文件中搜索指定字符串.
> 一位 Rust 社区的成员，Andrew Gallant，已经创建了一个功能完整且非常快速的 grep 版本，叫做 ripgrep
>

### 接受命令行参数
> 一如既往使用 cargo new 新建一个项目，我们称之为 minigrep 以便与可能已经安装在系统上的 grep 工具相区别：


```
% cargo new minigrep
     Created binary (application) `minigrep` package
%  cd minigrep
```

### 读取参数值
> 确保 minigrep 能够获取传递给它的命令行参数的值.我们需要一个 Rust 标准库提供的函数，也就是 std::env::args。这个函数返回一个传递给程序的命令行参数的 迭代器
> cargo run needle haystack 是一个命令行指令，用于在 Rust 编程语言中运行程序。其中 needle 和 haystack 是程序需要的参数。needle 代表需要查找的字符串，haystack 代表需要在其中查找的字符串集合。




### 读取文件
与src 同级创建 poem.txt
```
cargo run the poem.txt
```


### 重构改进模块性和错误处理

> 第一，main 现在进行了两个任务：它解析了参数并打开了文件,最好能分离出功能以便每个函数就负责一个任务.
> 第二个问题:query 和 filename 是程序中的配置变量,而像 contents 则用来执行程序逻辑
> 第三个问题是如果打开文件失败我们使用 expect 来打印出错误信息，不过这个错误信息只是说 Something went wrong reading the file.
> 第四:我们不停地使用 expect 来处理不同的错误，如果用户没有指定足够的参数来运行程序，他们会从 Rust 得到 index out of bounds 错误，而这并不能明确地解释问题.


#### 二进制项目的关注分离
- main 函数负责多个任务的组织问题在许多二进制项目中很常见
- 所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导性过程
- > - 将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。
- > -  当命令行解析逻辑比较小时，可以保留在 main.rs 中。
- > - 当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

+ 经过这些过程之后保留在 main 函数中的责任应该被限制为：

> + 使用参数值调用命令行解析逻辑
> + 设置任何其他的配置
> + 调用 lib.rs 中的 run 函数
> + 如果 run 返回错误，则处理这个错误


### 提取参数解析器

### 组合配置值
> 现在函数返回一个元组，不过立刻又将元组拆成了独立的部分


### 使用 clone 的权衡取舍
> 由于其运行时消耗，许多 Rustacean 之间有一个趋势是倾向于避免使用 clone 来解决所有权问题。在关于迭代器的第 13 章中，我们将会学习如何更有效率的处理这种情况，不过现在，复制一些字符串来取得进展是没有问题的，因为只会进行一次这样的拷贝，而且文件名和要搜索的字符串都比较短。在第一轮编写时拥有一个可以工作但有点低效的程序要比尝试过度优化代码更好一些。随着你对 Rust 更加熟练，将能更轻松的直奔合适的方法，不过现在调用 clone 是完全可以接受的。


### 创建一个 Config 的构造函数
```
impl Config{
     fn new(args:&[String]) -> Config{
          let query = args[0].clone();
          leyt filename = args[1].clone();
          Config{query,filename}
     }
}
```


### 修复错误处理
### 改善错误信息
> 在 new 函数中增加了一个检查在访问索引 1 和 2 之前检查 slice 是否足够长。如果 slice 不够长，我们使用一个更好的错误信息 panic 而不是 index out of bounds 信息：

### 从 new 中返回 Result 而不是调用 panic!


### 将错误信息输出到标准错误而不是标准输出

> 目前为止，我们将所有的输出都 println! 到了终端
> 标准输出（standard output，stdout）对应一般信息

> 标准错误（standard error，stderr）则用于错误信息.这种区别允许用户选择将程序正常输出定向到一个文件中并仍将错误信息打印到屏幕上

> 但是 println! 函数只能够打印到标准输出，所以我们必须使用其他方法来打印到标准错误。
> 
