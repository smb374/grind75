// Created by Po-Yeh Chen at 2024/12/24 17:21
// leetgo: 1.4.11
// https://leetcode.com/problems/01-matrix/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // BFS
        let m = mat.len();
        let n = mat[0].len();
        let mut queue = VecDeque::with_capacity(m * n);

        // initialize ans by marking the zero cells and others to i32::MAX
        let mut ans = vec![vec![i32::MAX; n]; m];
        for (i, row) in mat.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == 0 {
                    ans[i][j] = 0;
                    queue.push_front((i, j));
                }
            }
        }

        let deltas: [usize; 5] = [0, 1, 0, !0, 0]; // use wrapping add such that x + !0 = x - 1
        while let Some((i, j)) = queue.pop_back() {
            // explore neighbors
            // use windows to get a sliding window size of 2
            for d in deltas.windows(2) {
                let di = i.wrapping_add(d[0]);
                let dj = j.wrapping_add(d[1]);
                if di < m && dj < n && ans[di][dj] > ans[i][j] {
                    // Not visited cells or not optimal cells
                    ans[di][dj] = ans[i][j] + 1;
                    queue.push_front((di, dj));
                }
            }
        }

        ans
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mat: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::update_matrix(mat).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
