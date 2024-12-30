// Created by Po-Yeh Chen at 2024/12/27 23:15
// leetgo: 1.4.11
// https://leetcode.com/problems/subsets/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

fn routine(i: usize, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, nums: &[i32]) {
    if i == nums.len() {
        result.push(path.clone());
    } else {
        routine(i + 1, path, result, nums);
        path.push(nums[i]);
        routine(i + 1, path, result, nums);
        path.pop();
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();

        routine(0, &mut path, &mut result, &nums);

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::subsets(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
