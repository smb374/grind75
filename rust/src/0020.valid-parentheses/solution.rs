// Created by Po-Yeh Chen at 2024/12/23 11:39
// leetgo: 1.4.11
// https://leetcode.com/problems/valid-parentheses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

fn check_valid(lhs: Option<&char>, rhs: &char) -> bool {
    match lhs {
        Some(&'(') => *rhs == ')',
        Some(&'[') => *rhs == ']',
        Some(&'{') => *rhs == '}',
        _ => false,
    }
}

fn is_left(x: char) -> bool {
    x == '(' || x == '[' || x == '{'
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Use a stack to push.
        let mut stack: Vec<char> = Vec::with_capacity(s.len() >> 1);
        for c in s.chars() {
            if is_left(c) {
                stack.push(c);
            } else if check_valid(stack.last(), &c) {
                // Remember to use last to detect empty stack.
                stack.pop();
            } else {
                return false;
            }
        }
        stack.is_empty()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_valid(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
