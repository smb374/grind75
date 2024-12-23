// Created by Po-Yeh Chen at 2024/12/23 10:53
// leetgo: 1.4.11
// https://leetcode.com/problems/two-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // iterate nums and momorize visited valu and index with memo
        // if we found (target - v) in memo, thats the answer.
        let mut result = vec![-1, -1];
        let mut memo = HashMap::with_capacity(nums.len());
        for (i, &v) in nums.iter().enumerate() {
            let diff = target - v;
            if let Some(&j) = memo.get(&diff) {
                result[0] = j as i32;
                result[1] = i as i32;
                break;
            }
            memo.insert(v, i);
        }
        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
