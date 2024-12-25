// Created by Po-Yeh Chen at 2024/12/25 01:01
// leetgo: 1.4.11
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::ops::{Add, Div, Mul, Sub};

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        // Reverse Polish Notation
        // - push to stack if number
        // - pop 2 numbers then push operated result if operator.
        let mut stack = Vec::with_capacity(tokens.len());
        for tok in tokens {
            match tok.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().expect("Malformed expression");
                    let a = stack.pop().expect("Maiformed expression");

                    let op = match tok.as_str() {
                        "+" => i32::add,
                        "-" => i32::sub,
                        "*" => i32::mul,
                        "/" => i32::div,
                        _ => i32::add,
                    };

                    stack.push(op(a, b));
                }
                num => stack
                    .push(i32::from_str_radix(num, 10).expect("Should be numbers in this branch.")),
            }
        }

        stack.pop().unwrap()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let tokens: Vec<String> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::eval_rpn(tokens).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
