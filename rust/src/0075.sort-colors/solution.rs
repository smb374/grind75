// Created by Po-Yeh Chen at 2024/12/26 15:08
// leetgo: 1.4.11
// https://leetcode.com/problems/sort-colors/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // 2-pass buckets
        // let mut zeroes = 0;
        // let mut ones = 0;
        //
        // for &i in nums.iter() {
        //     match i {
        //         0 => zeroes += 1,
        //         1 => ones += 1,
        //         2 => {}
        //         _ => unreachable!(),
        //     }
        // }
        //
        // for i in 0..nums.len() {
        //     if zeroes > 0 {
        //         nums[i] = 0;
        //         zeroes -= 1;
        //     } else if ones > 0 {
        //         nums[i] = 1;
        //         ones -= 1;
        //     } else {
        //         nums[i] = 2;
        //     }
        // }
        // 3 pointers
        let mut lo = 0; // 0: 0..lo
        let mut hi = nums.len() - 1; // 2: hi..nums.len()
        let mut i = 0; // 1: lo..hi

        while i <= hi {
            if nums[i] == 0 {
                nums.swap(i, lo);
                lo += 1;
                i += 1;
            } else if nums[i] == 2 {
                nums.swap(i, hi);
                if hi == 0 {
                    break;
                }
                hi -= 1;
            } else {
                i += 1;
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    Solution::sort_colors(&mut nums);
    let ans: Vec<i32> = nums.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
