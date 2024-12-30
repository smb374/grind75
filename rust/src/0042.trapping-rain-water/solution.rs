// Created by Po-Yeh Chen at 2024/12/30 09:37
// leetgo: 1.4.11
// https://leetcode.com/problems/trapping-rain-water/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut total = 0;
        // pool_height: current maximum height in the boundary.
        let mut pool_height = 0;
        let (mut i, mut j) = (0, n - 1);

        while i < j {
            // Update the current maximum pool_height.
            pool_height = pool_height.max(height[i].min(height[j]));

            // add total with pool_height - lower end as water won't exceed the value
            if height[i] <= height[j] {
                total += 0.max(pool_height - height[i]);
                i += 1;
            } else {
                total += 0.max(pool_height - height[j]);
                j -= 1;
            }
        }

        total
    }
}

// @lc code=end

fn main() -> Result<()> {
    let height: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::trap(height).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
