// Created by Po-Yeh Chen at 2024/12/27 23:58
// leetgo: 1.4.11
// https://leetcode.com/problems/unique-paths/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        if m == 0 || n == 0 {
            0
        } else {
            let mut path = vec![vec![0; n]; m];

            for i in 0..m {
                for j in 0..n {
                    if i == 0 || j == 0 {
                        path[i][j] = 1;
                    } else {
                        path[i][j] = path[i][j - 1] + path[i - 1][j];
                    }
                }
            }
            path[m - 1][n - 1]
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let m: i32 = deserialize(&read_line()?)?;
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::unique_paths(m, n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
