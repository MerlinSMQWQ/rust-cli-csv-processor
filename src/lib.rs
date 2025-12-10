/*
 * @Author: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @Date: 2025-12-10 21:27:18
 * @LastEditors: MerlinSMQWQ MerlinSMQWQ@proton.me
 * @LastEditTime: 2025-12-10 22:05:26
 * @FilePath: \rust-cli-csv-processor\src\lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
mod opts;
mod prosess;

pub use opts::{ Opts, SubCommand, CsvOpts };
pub use prosess::{ User, process_csv };