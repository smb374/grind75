// Created by Po-Yeh Chen at 2024/12/25 17:11
// leetgo: 1.4.11
// https://leetcode.com/problems/min-stack/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

struct MinStack {
    data: Vec<i32>,
    previous_min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            data: vec![i32::MAX],
            previous_min: vec![i32::MAX],
        }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        self.previous_min.push(self.get_min().min(val))
    }

    fn pop(&mut self) {
        self.previous_min.pop();
        self.data.pop();
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.previous_min.last().unwrap()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = MinStack::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "push" => {
                let method_params = split_array(&params[i])?;
                let val: i32 = deserialize(&method_params[0])?;
                obj.push(val);
                output.push("null".to_string());
            }
            "pop" => {
                obj.pop();
                output.push("null".to_string());
            }
            "top" => {
                let ans: i32 = obj.top().into();
                output.push(serialize(ans)?);
            }
            "getMin" => {
                let ans: i32 = obj.get_min().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
