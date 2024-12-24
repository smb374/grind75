// Created by Po-Yeh Chen at 2024/12/24 11:07
// leetgo: 1.4.11
// https://leetcode.com/problems/climbing-stairs/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 {
            n
        } else {
            let mut p1 = 3;
            let mut p2 = 2;
            let mut cur = 0;

            for _ in 4..=n {
                cur = p1 + p2;
                p2 = p1;
                p1 = cur;
            }

            cur
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::climb_stairs(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
