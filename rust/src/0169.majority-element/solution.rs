// Created by Po-Yeh Chen at 2024/12/24 11:52
// leetgo: 1.4.11
// https://leetcode.com/problems/majority-element/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
// use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // Naive 0 ms
        // let occurrence = nums
        //     .iter()
        //     .fold(HashMap::with_capacity(nums.len()), |mut m, &x| {
        //         m.entry(x).and_modify(|v| *v += 1).or_insert(1);
        //         m
        //     });
        //
        // for (k, v) in occurrence.into_iter() {
        //     if v > (nums.len() as i32 >> 1) {
        //         return k;
        //     }
        // }
        // -1

        // Boyer-Moore Majority Vote
        let mut cnt = 1;
        let mut res = nums[0];

        for &v in nums.iter().skip(1) {
            if v == res {
                cnt += 1;
            } else {
                if cnt == 0 {
                    res = v;
                    cnt = 1;
                } else {
                    cnt -= 1;
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::majority_element(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
