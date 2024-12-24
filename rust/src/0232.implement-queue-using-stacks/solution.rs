// Created by Po-Yeh Chen at 2024/12/23 20:12
// leetgo: 1.4.11
// https://leetcode.com/problems/implement-queue-using-stacks/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

struct MyQueue {
    front: Vec<i32>,
    rear: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            front: Vec::new(),
            rear: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.rear.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.front.is_empty() {
            while let Some(x) = self.rear.pop() {
                self.front.push(x);
            }
        }
        self.front.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.front.is_empty() {
            while let Some(x) = self.rear.pop() {
                self.front.push(x);
            }
        }
        *self.front.last().unwrap()
    }

    fn empty(&mut self) -> bool {
        self.front.is_empty() && self.rear.is_empty()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = MyQueue::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "push" => {
                let method_params = split_array(&params[i])?;
                let x: i32 = deserialize(&method_params[0])?;
                obj.push(x);
                output.push("null".to_string());
            }
            "pop" => {
                let ans: i32 = obj.pop().into();
                output.push(serialize(ans)?);
            }
            "peek" => {
                let ans: i32 = obj.peek().into();
                output.push(serialize(ans)?);
            }
            "empty" => {
                let ans: bool = obj.empty().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
