// Created by Po-Yeh Chen at 2024/12/23 12:36
// leetgo: 1.4.11
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        let mut pmin = i32::MAX;
        let mut pmax = i32::MIN;

        for p in prices {
            pmin = std::cmp::min(p, pmin);
            pmax = std::cmp::max(p - pmin, pmax);
        }

        pmax
    }
}

// @lc code=end

fn main() -> Result<()> {
    let prices: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_profit(prices).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
