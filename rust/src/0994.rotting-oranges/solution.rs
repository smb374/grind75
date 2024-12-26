// Created by Po-Yeh Chen at 2024/12/25 20:06
// leetgo: 1.4.11
// https://leetcode.com/problems/rotting-oranges/

use std::i32;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut count = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Use multi-source BFS
        // Push sources to queue
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    queue.push_front((0, i, j));
                }
            }
        }

        // Run BFS. Use visited to store traversed cells.
        while let Some((min, i, j)) = queue.pop_back() {
            if visited.contains(&(i, j)) || grid[i][j] == 0 {
                continue;
            }
            visited.insert((i, j));
            grid[i][j] = 2;
            count = count.max(min);

            for d in [0, 1, 0, !0, 0].windows(2) {
                let di = i.wrapping_add(d[0]);
                let dj = j.wrapping_add(d[1]);

                if di < m && dj < n {
                    queue.push_front((min + 1, di, dj));
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    return -1;
                }
            }
        }

        count
    }
}

// @lc code=end

fn main() -> Result<()> {
    let grid: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::oranges_rotting(grid).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
