// Created by Po-Yeh Chen at 2024/12/29 14:56
// leetgo: 1.4.11
// https://leetcode.com/problems/find-all-anagrams-in-a-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_byt = s.as_bytes();
        let p_byt = p.as_bytes();
        let p_freq = p_byt.iter().fold([0i32; 26], |mut acc, x| {
            acc[(x - 0x61) as usize] += 1;
            acc
        });

        let mut i = 0;
        let mut starts = Vec::new();
        let mut window_freq = [0i32; 26];

        for j in 0..s_byt.len() {
            window_freq[(s_byt[j] - 0x61) as usize] += 1;
            if j - i + 1 == p_byt.len() {
                if window_freq == p_freq {
                    starts.push(i as i32);
                }
                window_freq[(s_byt[i] - 0x61) as usize] -= 1;
                i += 1;
            }
        }

        starts
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let p: String = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::find_anagrams(s, p).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
