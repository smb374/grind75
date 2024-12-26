// Created by Po-Yeh Chen at 2024/12/26 11:57
// leetgo: 1.4.11
// https://leetcode.com/problems/merge-intervals/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut idx = 0;
        let mut result = Vec::new();
        // Sort the intervals by start time
        intervals.sort_by(|x, y| x[0].cmp(&y[0]));
        // Push the first interval to result.
        result.push(intervals[0].clone());

        // merge loop
        for i in 1..intervals.len() {
            // if interval[i] is able to merge with result[idx]
            if intervals[i][0] <= result[idx][1] {
                // merge interval with result[idx]
                result[idx][1] = result[idx][1].max(intervals[i][1]);
            } else {
                // push interval to results and add idx
                result.push(intervals[i].clone());
                idx += 1;
            }
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let intervals: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::merge(intervals).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
