// Created by Po-Yeh Chen at 2024/12/29 19:20
// leetgo: 1.4.11
// https://leetcode.com/problems/task-scheduler/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
impl Solution {
    // For 1 element: A has frequency of k
    // -> A xxxx A xxxx ... A xxxx A
    // k - 1 groups of A xxxx, each of length n + 1, + 1 final A
    // -> (k - 1) * (n + 1) + 1
    // for m items that has the maximum frequence k: (k - 1) * (n + 1) + m
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let res = tasks.len() as i32;
        let mut freq = [0; 26];

        for &t in tasks.iter() {
            let idx = t as usize - 'A' as usize;
            freq[idx] += 1;
        }

        let max = *freq.iter().max().unwrap();
        let num_max = freq.iter().filter(|&&x| x == max).count();
        res.max((n + 1) * (max - 1) + num_max as i32)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let tasks: Vec<char> = deserialize(&read_line()?)?;
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::least_interval(tasks, n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
