// Created by Po-Yeh Chen at 2024/12/26 11:23
// leetgo: 1.4.11
// https://leetcode.com/problems/combination-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
fn routine(
    index: usize,
    target: i32,
    candidates: &[i32],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    // Base case: added up to candidates -> 1 combination.
    if target == 0 {
        result.push(current.clone());
        return;
    }

    // For each rest of the candidates
    for i in index..candidates.len() {
        if candidates[i] <= target {
            current.push(candidates[i]);
            // find next to push into current in i .. candidates.len() (rest of the candidates)
            routine(i, target - candidates[i], candidates, current, result);
            // backtrack
            current.pop();
        }
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        routine(0, target, &candidates, &mut current, &mut result);
        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let candidates: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::combination_sum(candidates, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
