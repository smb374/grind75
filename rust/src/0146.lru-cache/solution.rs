// Created by Po-Yeh Chen at 2024/12/28 17:03
// leetgo: 1.4.11
// https://leetcode.com/problems/lru-cache/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::collections::{BTreeMap, BinaryHeap, HashMap, VecDeque};

struct LRUCache {
    data: HashMap<i32, i32>,
    mgmt: VecDeque<i32>,
    cap: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            data: HashMap::with_capacity(capacity as usize),
            mgmt: VecDeque::with_capacity(capacity as usize),
            cap: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.data.get(&key) {
            let index = self.mgmt.iter().position(|&k| k == key).unwrap();
            self.mgmt.remove(index);
            self.mgmt.push_front(key);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.data.contains_key(&key) {
            let index = self.mgmt.iter().position(|&k| k == key).unwrap();
            self.mgmt.remove(index);
            self.mgmt.push_front(key);
            self.data.entry(key).and_modify(|e| *e = value);
        } else {
            if self.data.len() == self.cap {
                let last = self.mgmt.pop_back().unwrap();
                self.data.remove(&last);
            }
            self.data.insert(key, value);
            self.mgmt.push_front(key);
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let capacity: i32 = deserialize(&constructor_params[0])?;
    #[allow(unused_mut)]
    let mut obj = LRUCache::new(capacity);

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "get" => {
                let method_params = split_array(&params[i])?;
                let key: i32 = deserialize(&method_params[0])?;
                let ans: i32 = obj.get(key).into();
                output.push(serialize(ans)?);
            }
            "put" => {
                let method_params = split_array(&params[i])?;
                let key: i32 = deserialize(&method_params[0])?;
                let value: i32 = deserialize(&method_params[1])?;
                obj.put(key, value);
                output.push("null".to_string());
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
