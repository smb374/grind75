// Created by Po-Yeh Chen at 2024/12/23 16:33
// leetgo: 1.4.11
// https://leetcode.com/problems/binary-search/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len();
        while low < high {
            let mid = low + ((high - low) >> 1);
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        -1
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::search(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
