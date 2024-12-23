// Created by Po-Yeh Chen at 2024/12/23 16:23
// leetgo: 1.4.11
// https://leetcode.com/problems/valid-anagram/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut smap = [0; 26];
        let mut tmap = [0; 26];
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        (0..s.len()).for_each(|i| {
            smap[sb[i] as usize - 0x61] += 1;
            tmap[tb[i] as usize - 0x61] += 1;
        });
        smap.iter()
            .zip(tmap.iter())
            .fold(true, |acc, (a, b)| acc && (a == b))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_anagram(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
