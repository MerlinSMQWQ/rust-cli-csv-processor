/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-12-09 23:25:13
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-10 18:09:05
 * @FilePath: \rust-cli-csv-processor\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// cli命令样例：rcli csv -i input.csv -o output.json --header --d ','
use clap::{Parser};

// 为结构体实现 Debug 和 Parser trait
#[derive(Debug, Parser)]
// 配置程序元数据，name是程序的名字，version是直接使用Cargo.toml中version的值，author也是，about是简短的程序描述，long_about是详细说明，但是这里设置为None
#[command(name = "rust-csv-processor", version, author, about="This is merely a practice.", long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

// clap拥有subcommand的概念，可以实现git push这样的多层次的命令，push是git的子命令，每个子命令都可以有自己的参数
#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats.")]
    Csv(CsvOpts)
}

#[derive(Debug, Parser)]
struct  CsvOpts {
    #[arg(short, long)]
    input: String,
    
    /// `default_value` 用于直接提供一个字符串形式的默认值，适用于可以直接从字符串解析为目标类型的参数，例如是"output.json" 是一个字符串，默认值会根据目标类型自动解析。
    #[arg(short, long, default_value="output.json")]
    output: String,

    #[arg(short, long, default_value_t = 't')]
    delimiter: char,

    /// `default_value_t` 用于提供一个具体的目标类型值，通常是一个实现了Defaut trait或者通过表达式生成的值，适合复杂类型或者需要显示构造默认值的情况，比如这里的true，就不需要任何转换，可以直接作为header的值
    /// 这里如果使用header的short，就要自己规避一下使用'h'，因为help默认的short也是'h'，这里可以改成'H'或者删掉short
    #[arg(short='H', long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
