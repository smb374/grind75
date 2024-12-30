// Created by Po-Yeh Chen at 2024/12/28 15:40
// leetgo: 1.4.11
// https://leetcode.com/problems/word-search/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Remember that we can mutate the board to perform memoization.
fn dfs(
    (r, c): (usize, usize),
    depth: usize,
    board: &mut Vec<Vec<char>>,
    target: &[u8],
    nrows: usize,
    ncols: usize,
) -> bool {
    if depth == target.len() {
        true
    } else if board[r][c] as u8 != target[depth] {
        false
    } else {
        let val = board[r][c];
        board[r][c] = '#';
        let mut result = false;
        for d in [0, 1, 0, !0, 0].windows(2) {
            let nr = r.wrapping_add(d[0]);
            let nc = c.wrapping_add(d[1]);
            if nr < nrows && nc < ncols {
                result = dfs((nr, nc), depth + 1, board, target, nrows, ncols);
                if result {
                    break;
                }
            }
        }
        board[r][c] = val;
        result
    }
}

fn check_frequency(mut freq: [i32; 128], word: &[u8]) -> bool {
    for &c in word {
        freq[c as usize] -= 1;
    }

    freq.into_iter().all(|x| x >= 0)
}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let nrows = board.len();
        let ncols = board[0].len();
        let byt = word.as_bytes();
        let mut freq = [0; 128];

        let mut starts = Vec::new();

        for (i, row) in board.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                freq[c as usize] += 1;
                if c as u8 == byt[0] {
                    starts.push((i, j));
                }
            }
        }

        if byt.len() > nrows * ncols {
            return false;
        }
        if !check_frequency(freq, byt) {
            return false;
        }
        if byt.len() == 1 {
            return board.iter().any(|r| r.contains(&(byt[0] as char)));
        }

        for cell in starts {
            if dfs(cell, 0, &mut board, byt, nrows, ncols) {
                return true;
            }
        }
        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let board: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let word: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::exist(board, word).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
