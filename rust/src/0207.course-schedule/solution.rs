// Created by Po-Yeh Chen at 2024/12/25 10:52
// leetgo: 1.4.11
// https://leetcode.com/problems/course-schedule/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Mark {
    Todo,
    Temp,
    Permanent,
}

fn visit(n: usize, visited: &mut [Mark], adj: &HashMap<usize, Vec<usize>>) -> bool {
    match visited[n] {
        Mark::Permanent => false,
        Mark::Temp => true,
        Mark::Todo => {
            visited[n] = Mark::Temp;
            if adj
                .get(&n)
                .map(|ns| ns.iter().any(|&m| visit(m, visited, adj)))
                .unwrap_or(false)
            {
                return true;
            }
            visited[n] = Mark::Permanent;
            false
        }
    }
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut visited = vec![Mark::Todo; num_courses];
        let mut adj: HashMap<usize, Vec<usize>> = HashMap::with_capacity(num_courses as usize);

        // Build adj list
        for p in prerequisites {
            adj.entry(p[0] as usize)
                .and_modify(|v| v.push(p[1] as usize))
                .or_insert(vec![p[1] as usize]);
        }

        // DFS
        (0..num_courses).all(|n| !visit(n, &mut visited, &adj))
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num_courses: i32 = deserialize(&read_line()?)?;
    let prerequisites: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: bool = Solution::can_finish(num_courses, prerequisites).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
