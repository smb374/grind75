// Created by Po-Yeh Chen at 2024/12/24 14:58
// leetgo: 1.4.11
// https://leetcode.com/problems/contains-duplicate/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // make use of all's short circuit effect.
        let mut visited = HashSet::with_capacity(nums.len());
        !nums.into_iter().all(|x| visited.insert(x))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: bool = Solution::contains_duplicate(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
