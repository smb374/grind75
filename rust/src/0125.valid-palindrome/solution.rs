// Created by Po-Yeh Chen at 2024/12/23 14:00
// leetgo: 1.4.11
// https://leetcode.com/problems/valid-palindrome/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered: String = s
            .to_lowercase()
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .collect();
        let is_odd = (filtered.len() & 1) == 1;
        let half = filtered.len() >> 1;
        let (front, back) = if is_odd {
            (&filtered[..half], &filtered[half + 1..])
        } else {
            (&filtered[..half], &filtered[half..])
        };
        front
            .bytes()
            .zip(back.bytes().rev())
            .fold(true, |acc, (a, b)| acc && (a == b))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_palindrome(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
