// Created by Po-Yeh Chen at 2024/12/27 21:49
// leetgo: 1.4.11
// https://leetcode.com/problems/partition-equal-subset-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // 0/1 Knapsack
        let total: i32 = nums.iter().sum();
        if total & 1 == 1 {
            return false;
        }
        // target sum
        let target = (total >> 1) as usize;
        let mut mem = vec![false; target + 1];
        mem[0] = true;
        // for each number
        for &n in nums.iter() {
            // i from target down to n
            for i in (n as usize..=target).rev() {
                // for subproblem i, the result is taking n or not
                mem[i] = mem[i] || mem[i - n as usize];
            }
            if mem[target] {
                return true;
            }
        }
        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: bool = Solution::can_partition(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
