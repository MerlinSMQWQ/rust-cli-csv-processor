use std::fs;
use serde::{Deserialize, Serialize};
use anyhow;
use crate::{CsvOpts};


// 定义一个数据结构用于存放CSV解析以后的数据
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub nickname: String,
    pub id: usize,
}

pub fn process_csv(opts: CsvOpts) -> anyhow::Result<()> {
    // 这里的 ? 相当于使用match处理一个Result<foo>类型的值
    let mut reader = csv::Reader::from_path(opts.input)?;
    // with_capacity用于初始化容器的长度，预先分配500个位置，除非后续元素个数超过500，否则不会在那之前出发重分配，提高了性能
    let mut ret: Vec<User> = Vec::with_capacity(500);
    
    for result in reader.deserialize() {
        let record: User = result?;
        ret.push(record);
    }
    
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(opts.output, json)?;
    Ok(())
}