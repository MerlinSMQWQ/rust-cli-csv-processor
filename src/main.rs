/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-12-09 23:25:13
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-10 22:14:51
 * @FilePath: \rust-cli-csv-processor\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// cli命令样例：rcli csv -i input.csv -o output.json --header --d ','
use rust_cli_csv_processor::{Opts, SubCommand, process_csv };
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(opts)?;
        }
    }

    anyhow::Ok(())
}
