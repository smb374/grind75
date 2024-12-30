// Created by Po-Yeh Chen at 2024/12/27 22:23
// leetgo: 1.4.11
// https://leetcode.com/problems/string-to-integer-atoi/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // let trimmed = s.trim_start();
        // let (rest, sign) = match trimmed.strip_prefix('-') {
        //     Some(s) => (s, -1),
        //     None => (trimmed.strip_prefix('+').unwrap_or(trimmed), 1),
        // };
        //
        // rest.chars()
        //     .map(|c| c.to_digit(10))
        //     .take_while(|x| x.is_some())
        //     .flatten()
        //     .fold(0, |acc, digit| {
        //         acc.saturating_mul(10).saturating_add(sign * digit as i32)
        //     })
        let chars: Vec<char> = s.chars().skip_while(|&c| c == ' ').collect();
        let (start, sign) = match chars.get(0) {
            Some('+') => (1, 1),
            Some('-') => (1, -1),
            Some(_) => (0, 1),
            None => return 0,
        };

        let mut acc: i32 = 0;

        for i in start..chars.len() {
            match chars[i].to_digit(10) {
                None => break,
                Some(d) => {
                    acc = acc.saturating_mul(10).saturating_add(sign * d as i32);
                }
            }
        }

        acc
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::my_atoi(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
