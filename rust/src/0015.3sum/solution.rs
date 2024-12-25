// Created by Po-Yeh Chen at 2024/12/24 21:16
// leetgo: 1.4.11
// https://leetcode.com/problems/3sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::cmp::Ordering;
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Sort and then use two pointer.
        nums.sort();
        let mut res = HashSet::new();

        for i in 0..nums.len() - 2 {
            // Skip same numbers
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let x = nums[i];
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let y = nums[j];
                let z = nums[k];

                // Standard two pointer moving
                let sum = x + y + z;
                match sum.cmp(&0) {
                    Ordering::Less => {
                        j += 1;
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
                    Ordering::Equal => {
                        res.insert(vec![x, y, z]);

                        // skip same numbers for j & k in this i loop.
                        while j < k && nums[j] == y {
                            j += 1;
                        }
                        while j < k && nums[k] == z {
                            k -= 1;
                        }
                    }
                }
            }
        }

        res.into_iter().collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::three_sum(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
