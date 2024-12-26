// Created by Po-Yeh Chen at 2024/12/26 11:39
// leetgo: 1.4.11
// https://leetcode.com/problems/permutations/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashSet;

fn routine(
    nums: &[i32],
    visited: &mut HashSet<usize>,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    // Base case: used all indices
    if visited.len() == nums.len() {
        result.push(current.clone());
    }

    for i in 0..nums.len() {
        // For every non-visited indices
        if !visited.contains(&i) {
            // push the number to current and index to visited.
            current.push(nums[i]);
            visited.insert(i);
            // Find next
            routine(nums, visited, current, result);
            // Backtrack
            visited.remove(&i);
            current.pop();
        }
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut visited = HashSet::with_capacity(nums.len());
        let mut current = Vec::with_capacity(nums.len());
        let mut result = Vec::new();

        routine(&nums, &mut visited, &mut current, &mut result);

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::permute(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
