// Created by Po-Yeh Chen at 2024/12/24 10:49
// leetgo: 1.4.11
// https://leetcode.com/problems/ransom-note/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut tab = [0i32; 26];

        magazine.bytes().for_each(|b| {
            tab[b as usize - 0x61] += 1;
        });

        ransom_note.bytes().for_each(|b| {
            tab[b as usize - 0x61] -= 1;
        });

        !tab.into_iter().any(|x| x < 0)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ransom_note: String = deserialize(&read_line()?)?;
    let magazine: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::can_construct(ransom_note, magazine).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
