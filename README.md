# rust-cli-csv-processor

This is merely a practice.

# Rust如何做一个CLI
Rust写CLI是相当合适的，我们有很多选择，比如这次我们要使用clap，除了clap以外，我们还会使用structopt、argh、pico-args等，并且我们还可能会使用console对终端输出进行美化

现在我们使用clap crate，使用命令
```bash
cargo add clap --features derive
```
rust的一个crate下可能有多个features，通常我们只需要使用到一个或者几个feature，我们就可以使用 `--features` 选项来进行选择，这样做也可以控制最后的编译得到的二进制文件的大小，也可以在一定程度上减轻编译的负担。