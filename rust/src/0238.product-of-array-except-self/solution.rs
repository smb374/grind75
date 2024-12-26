// Created by Po-Yeh Chen at 2024/12/25 16:48
// leetgo: 1.4.11
// https://leetcode.com/problems/product-of-array-except-self/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.iter().all(|&x| x == 1) {
            return vec![1; nums.len()];
        }
        // Single loop with tuple
        // let mut store: Vec<(i32, i32)> = vec![(1, 1); nums.len()];
        // for i in 1..nums.len() {
        //     let j = nums.len() - i - 1;
        //     store[i].0 = store[i - 1].0 * nums[i - 1];
        //     store[j].1 = store[j + 1].1 * nums[j + 1];
        // }
        //
        // store.into_iter().map(|(x, y)| x * y).collect()

        // O(1) space
        let mut result = vec![1; nums.len()];
        let mut prefix = 1;
        let mut suffix = 1;

        for i in 1..nums.len() {
            let j = nums.len() - i - 1;
            prefix *= nums[i - 1];
            suffix *= nums[j + 1];
            result[i] *= prefix;
            result[j] *= suffix;
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::product_except_self(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
