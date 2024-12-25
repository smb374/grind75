// Created by Po-Yeh Chen at 2024/12/24 16:33
// leetgo: 1.4.11
// https://leetcode.com/problems/insert-interval/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // binary search & splicing
        // e.g.: x = [[1,2],[3,5],[6,7],[8,10],[12,16]], ni = [4, 8]
        // using partition_point to find the partition point (first index of the second partition)
        // i.e. the index of the first item that makes the predicate false.
        //
        // Get the interval index with predicate i.end < ni.start
        // left = 1 as x[1][1] = 5 >= ni[0] = 4
        let left = intervals.partition_point(|x| x[1] < new_interval[0]);
        // Get the interval index with predicate i.start <= ni.end
        // right = 4 as x[4][0] = 12 > ni[1] = 8
        let right = intervals.partition_point(|x| x[0] <= new_interval[1]);

        // replace start and end of new.
        if left < intervals.len() {
            // ni[0] = min(ni[0] = 4, x[left][0] = 3) = 3
            new_interval[0] = new_interval[0].min(intervals[left][0]);
        }
        if right >= 1 {
            // ni[1] = max(ni[1] = 8, x[right-1][1] = 10) = 10
            new_interval[1] = new_interval[1].max(intervals[right - 1][1]);
        }

        // replace the region with the new interval
        intervals.splice(left..right, [new_interval]);
        intervals
    }
}

// @lc code=end

fn main() -> Result<()> {
    let intervals: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let new_interval: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::insert(intervals, new_interval).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
