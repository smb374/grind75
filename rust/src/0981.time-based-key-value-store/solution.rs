// Created by Po-Yeh Chen at 2024/12/26 12:55
// leetgo: 1.4.11
// https://leetcode.com/problems/time-based-key-value-store/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data
            .entry(key)
            .and_modify(|v| v.push((timestamp, value.clone())))
            .or_insert(vec![(timestamp, value)]);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.data.get(&key) {
            None => String::new(),
            Some(v) => {
                let idx = v.partition_point(|x| x.0 <= timestamp).wrapping_sub(1);
                v.get(idx).map(|v| v.1.clone()).unwrap_or(String::new())
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = TimeMap::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "set" => {
                let method_params = split_array(&params[i])?;
                let key: String = deserialize(&method_params[0])?;
                let value: String = deserialize(&method_params[1])?;
                let timestamp: i32 = deserialize(&method_params[2])?;
                obj.set(key, value, timestamp);
                output.push("null".to_string());
            }
            "get" => {
                let method_params = split_array(&params[i])?;
                let key: String = deserialize(&method_params[0])?;
                let timestamp: i32 = deserialize(&method_params[1])?;
                let ans: String = obj.get(key, timestamp).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
