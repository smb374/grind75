// Created by Po-Yeh Chen at 2024/12/24 20:22
// leetgo: 1.4.11
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let byt = s.as_bytes();

        let mut max_len = 0;
        let mut pos = [0usize; 256];
        let mut start = 0;

        for (idx, &b) in byt.iter().enumerate() {
            // update starting position.
            start = start.max(pos[b as usize]);
            max_len = max_len.max(idx - start + 1);
            // update pos by storing potential start index.
            pos[b as usize] = idx + 1;
        }

        max_len as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::length_of_longest_substring(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
