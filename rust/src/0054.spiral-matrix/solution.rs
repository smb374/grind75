// Created by Po-Yeh Chen at 2024/12/27 22:37
// leetgo: 1.4.11
// https://leetcode.com/problems/spiral-matrix/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

fn change_direction(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (0, 1) => (1, 0),   // r -> d
        (1, 0) => (0, -1),  // d -> l
        (0, -1) => (-1, 0), // l -> u
        (-1, 0) => (0, 1),  // u -> r
        _ => unreachable!(),
    }
}

fn check_visited(cell: (usize, usize), visited: &Vec<Vec<bool>>) -> bool {
    visited
        .get(cell.0)
        .and_then(|v| v.get(cell.1))
        .map(|&x| x)
        .unwrap_or(true)
}

fn check(
    cell: (usize, usize),
    dir: (isize, isize),
    limit: (usize, usize),
    visited: &Vec<Vec<bool>>,
) -> bool {
    let next = (
        (cell.0 as isize + dir.0) as usize,
        (cell.1 as isize + dir.1) as usize,
    );

    next.0 >= limit.0 || next.1 >= limit.1 || check_visited(next, visited)
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut result = Vec::new();
        let mut visited = vec![vec![false; n]; m];
        let mut direction = (0, 1);
        let mut i = 0;
        let mut j = 0;

        while !check_visited((i, j), &visited) {
            result.push(matrix[i][j]);
            visited[i][j] = true;

            if check((i, j), direction, (m, n), &visited) {
                direction = change_direction(direction);
            }

            i = (i as isize + direction.0) as usize;
            j = (j as isize + direction.1) as usize;
        }

        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::spiral_order(matrix).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
