// Created by Po-Yeh Chen at 2024/12/23 23:51
// leetgo: 1.4.11
// https://leetcode.com/problems/first-bad-version/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;
        let mut ans = -1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if self.isBadVersion(mid) {
                ans = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        ans
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let bad: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::first_bad_version(n, bad).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
