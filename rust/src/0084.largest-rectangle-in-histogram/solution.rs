// Created by Po-Yeh Chen at 2024/12/29 19:41
// leetgo: 1.4.11
// https://leetcode.com/problems/largest-rectangle-in-histogram/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // monotonic stack
        let n = heights.len();
        let mut area: i32 = 0;
        let mut stack = Vec::new();
        stack.push((0, heights[0]));

        for i in 1..n {
            if stack.last().map(|&p| heights[i] < p.1).unwrap_or(false) {
                let mut tmp = 0;
                while let Some(&(idx, h)) = stack.last() {
                    if heights[i] < h {
                        let _ = stack.pop();
                        area = area.max(h * (i - idx) as i32);
                        tmp = idx;
                    } else {
                        break;
                    }
                }
                stack.push((tmp, heights[i]));
            } else {
                stack.push((i, heights[i]));
            }
        }

        while let Some((idx, h)) = stack.pop() {
            area = area.max(h * (n - idx) as i32);
        }

        area
    }
}

// @lc code=end

fn main() -> Result<()> {
    let heights: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::largest_rectangle_area(heights).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
