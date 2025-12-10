# rust-cli-csv-processor

This is merely a practice.

# Rust如何做一个CLI
Rust写CLI是相当合适的，我们有很多选择，比如这次我们要使用clap，除了clap以外，我们还会使用structopt、argh、pico-args等，并且我们还可能会使用console对终端输出进行美化

现在我们使用clap crate，使用命令
```bash
cargo add clap --features derive
```
rust的一个crate下可能有多个features，通常我们只需要使用到一个或者几个feature，我们就可以使用 `--features` 选项来进行选择，这样做也可以控制最后的编译得到的二进制文件的大小，也可以在一定程度上减轻编译的负担。

Rust构建CLI有两种方式，一种是直接通过一般的代码构建CLI，另一种是通过数据结构的声明，其中后者，也就是使用数据结构的方式是更为推荐的一种方式，`clap` 也是使用这种方式，这个项目也将采用这个方式

# Rust如何处理CSV数据
Rust有处理CSV文件的crate，他的名字就叫做csv，使用命令安装：
```
cargo add csv
```
csv主要有两种写法：
    - 基于记录的读写方式：`StringRecord` : 以字符串形式处理 CSV 记录，`ByteRecord` : 以字节形式处理 CSV 记录，适合简单的 CSV 处理需求，直接操作行数据
    - 基于 serde 的序列化方式：结合 `serde` crate 使用，可以将 CSV 数据直接序列化/反序列化为自定义的结构体，通过 `#[derive Deserialize, Serialize]` 实现，更适合结构化数据处理，类型安全更好
这里我们使用第二种写法。

# Rust如何将CSV文件解析为json文件
我们使用的是serde-json这个crate，这个crate非常强大，我们通常用它来处理json，这个项目中也是使用serde-json