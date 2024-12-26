// Created by Po-Yeh Chen at 2024/12/25 16:19
// leetgo: 1.4.11
// https://leetcode.com/problems/coin-change/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        // 1-D DP array
        let mut mem = vec![i32::MAX; amount + 1];
        // Base case: No changes -> 0 coins
        mem[0] = 0;

        // For each coin in coins
        for coin in coins {
            let c = coin as usize;
            // For each value i in coin..amount
            for i in c..=amount {
                // if mem[i - c] != i32::MAX, i.e. have solve for change amount (i - c).
                if mem[i - c] != i32::MAX {
                    // Update the minimum required amount coins with min(mem[i], mem[i-c])
                    mem[i] = mem[i].min(mem[i - c] + 1);
                }
            }
        }

        if mem[amount] == i32::MAX {
            -1
        } else {
            mem[amount]
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let coins: Vec<i32> = deserialize(&read_line()?)?;
    let amount: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::coin_change(coins, amount).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
