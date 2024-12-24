// Created by Po-Yeh Chen at 2024/12/24 11:29
// leetgo: 1.4.11
// https://leetcode.com/problems/longest-palindrome/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        if s.is_empty() {
            0
        } else if s.len() == 1 {
            1
        } else {
            let occurrence = s.chars().fold(HashMap::new(), |mut m, c| {
                let v = m.get(&c).map(|&x| x).unwrap_or(0);
                m.insert(c, v + 1);
                m
            });

            let mut size = 0;
            let mut odds = 0;

            for &v in occurrence.values() {
                if v & 1 == 1 {
                    odds += 1;
                    size += v - 1;
                } else {
                    size += v;
                }
            }

            if odds > 0 {
                size + 1
            } else {
                size
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::longest_palindrome(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
