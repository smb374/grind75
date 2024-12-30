// Created by Po-Yeh Chen at 2024/12/29 00:05
// leetgo: 1.4.11
// https://leetcode.com/problems/container-with-most-water/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut area = i32::MIN;

        while i < j {
            area = area.max((j - i) as i32 * height[i].min(height[j]));
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        area
    }
}

// @lc code=end

fn main() -> Result<()> {
    let height: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_area(height).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
