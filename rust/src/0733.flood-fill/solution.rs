// Created by Po-Yeh Chen at 2024/12/23 16:58
// leetgo: 1.4.11
// https://leetcode.com/problems/flood-fill/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::{HashSet, VecDeque};

fn in_bound((rmax, cmax): (usize, usize), (r, c): (usize, usize)) -> bool {
    r <= rmax && c <= cmax
}

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        // Nothing, return nothing.
        if image.len() == 0 {
            return Vec::new();
        }
        let root_col = image[sr as usize][sc as usize];
        let limit = (image.len() - 1, image[0].len() - 1);

        if root_col == color {
            return image;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Use a queue to visit cells
        // and visited to record visited cells.
        queue.push_front((sr as usize, sc as usize));

        while let Some((r, c)) = queue.pop_back() {
            if visited.contains(&(r, c)) {
                continue;
            }
            // Use saturating_sub s.t. r - 1 = 0 if r = 0
            [
                (r.saturating_sub(1), c),
                (r + 1, c),
                (r, c.saturating_sub(1)),
                (r, c + 1),
            ]
            .into_iter()
            .for_each(|(cr, cc)| {
                if in_bound(limit, (cr, cc))
                    && image[cr as usize][cc as usize] == root_col
                    && !visited.contains(&(cr, cc))
                {
                    queue.push_front((cr, cc));
                }
            });
            image[r][c] = color;
            visited.insert((r, c));
        }
        image
    }
}

// @lc code=end

fn main() -> Result<()> {
    let image: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let sr: i32 = deserialize(&read_line()?)?;
    let sc: i32 = deserialize(&read_line()?)?;
    let color: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::flood_fill(image, sr, sc, color).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
