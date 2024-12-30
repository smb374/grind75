// Created by Po-Yeh Chen at 2024/12/30 10:13
// leetgo: 1.4.11
// https://leetcode.com/problems/maximum-profit-in-job-scheduling/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

#[derive(Debug, Clone, Copy)]
struct Job {
    start: i32,
    end: i32,
    profit: i32,
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs: Vec<Job> =
            (0..start_time.len()).fold(Vec::with_capacity(n), |mut acc, idx| {
                acc.push(Job {
                    start: start_time[idx],
                    end: end_time[idx],
                    profit: profit[idx],
                });
                acc
            });
        // Sort the jobs by end time.
        jobs.sort_unstable_by_key(|x| x.end);
        let mut mem = vec![0; n + 1];

        for i in 0..n {
            // For each job, we need to find the closest job that finished before the current job's
            // start.
            // Using partition_point we can find the insertion point for current job's start time
            // such that jobs[0..j].all(|x| x.end <= jobs[i].start) && jobs[j..i].all(|x| x.end >
            // jobs[i].start).
            // => mem[j] represents corresponds to the maximum profit up to the job that finishes
            // right before the current job can start.
            let j = jobs[0..i].partition_point(|x| x.end <= jobs[i].start);
            // For next job, the maximum profit will be either current's maximum profit or maximum
            // profit after job j + current profit.
            mem[i + 1] = mem[i].max(mem[j] + jobs[i].profit);
        }

        mem[n]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let start_time: Vec<i32> = deserialize(&read_line()?)?;
    let end_time: Vec<i32> = deserialize(&read_line()?)?;
    let profit: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::job_scheduling(start_time, end_time, profit).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
