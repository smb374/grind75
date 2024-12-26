// Created by Po-Yeh Chen at 2024/12/25 19:04
// leetgo: 1.4.11
// https://leetcode.com/problems/number-of-islands/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

fn bfs_routine(grid: &mut [Vec<char>], i: usize, j: usize, m: usize, n: usize) {
    if i < m && j < n && grid[i][j] == '1' {
        grid[i][j] = '0';

        for d in [0, 1, 0, !0, 0].windows(2) {
            bfs_routine(grid, i.wrapping_add(d[0]), j.wrapping_add(d[1]), m, n);
        }
    }
}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    count += 1;
                    // Use BFS to mutate the grid to eliminate all connected '1's
                    bfs_routine(grid.as_mut(), i, j, m, n);
                }
            }
        }

        count
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let grid: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::num_islands(grid).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
