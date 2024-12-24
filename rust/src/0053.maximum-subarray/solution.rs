// Created by Po-Yeh Chen at 2024/12/24 15:12
// leetgo: 1.4.11
// https://leetcode.com/problems/maximum-subarray/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, i32::MIN), |(sum, ans), n| {
                let nsum = sum + n;
                // Prefer summing all of numbers unless the sum becomes negative at some point.
                // ans = current maximum sum -> sets subarray end
                // sum will reset to 0 when negative -> sets subarray start
                (nsum.max(0), ans.max(nsum))
            })
            .1
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_sub_array(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
