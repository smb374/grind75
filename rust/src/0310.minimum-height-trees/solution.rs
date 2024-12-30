// Created by Po-Yeh Chen at 2024/12/29 16:02
// leetgo: 1.4.11
// https://leetcode.com/problems/minimum-height-trees/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // Basically topological sort until 1 or 2 nodes left from leaf in the original graph.
        let n = n as usize;
        if n == 1 {
            return vec![0]; // Base case optimization
        }

        let mut graph = vec![Vec::new(); n];
        let mut degrees = vec![0; n];

        // Build graph and record each node's degrees.
        for edge in edges {
            let (p, q) = (edge[0] as usize, edge[1] as usize);
            graph[p].push(q);
            graph[q].push(p);
            degrees[p] += 1;
            degrees[q] += 1;
        }

        // Initialize queue with leaves (degree[n] == 1)
        let mut queue = VecDeque::new();
        for (i, &degree) in degrees.iter().enumerate() {
            if degree == 1 {
                queue.push_front(i);
            }
        }

        // Trim leaves until reaching the MHTs core
        let mut remain = n;
        while remain > 2 {
            let size = queue.len();
            remain -= size;
            for _ in 0..size {
                if let Some(p) = queue.pop_back() {
                    for &q in &graph[p] {
                        degrees[q] -= 1;
                        if degrees[q] == 1 {
                            queue.push_front(q);
                        }
                    }
                }
            }
        }

        // Convert remaining nodes (or leaves) to i32 for the result
        queue.iter().map(|&x| x as i32).collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let edges: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::find_min_height_trees(n, edges).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
