// Created by Po-Yeh Chen at 2024/12/30 12:21
// leetgo: 1.4.11
// https://leetcode.com/problems/basic-calculator/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn calculate(s: String) -> i32 {
        // Only +, - => we can use sign to determine if + or - then always add to the total
        // Use stack to store the sign and current total if we encoutered parentheses.
        let mut stack = Vec::new();
        let (mut sign, mut curr, mut total) = (1, 0, 0);

        stack.push(sign);

        for tok in s.chars() {
            match tok {
                // + | - => add curr * sign to the total, reset curr to 0 then set sign to + | -
                '+' => {
                    total += curr * sign;
                    curr = 0;
                    sign = 1;
                }
                '-' => {
                    total += curr * sign;
                    curr = 0;
                    sign = -1;
                }
                // Left parentheses: push current total and sign, reset total and sign for
                // calculating the expression in the parentheses.
                '(' => {
                    stack.push(total);
                    stack.push(sign);
                    total = 0;
                    sign = 1;
                }
                // Right parentheses: finalize expression result in parentheses and add to total,
                // apply sign from stack top, then add stored total from stack top.
                // Reset current and sign after.
                ')' => {
                    total += curr * sign;
                    total *= stack.pop().unwrap_or(1);
                    total += stack.pop().unwrap_or(0);
                    curr = 0;
                    sign = 1;
                }
                // Others: if digit: update curr; else: ignore
                c => {
                    if let Some(d) = c.to_digit(10) {
                        curr = curr * 10 + d as i32;
                    }
                }
            }
        }
        // add dangling values
        total + curr * sign
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::calculate(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
