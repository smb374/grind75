// Created by Po-Yeh Chen at 2024/12/26 10:37
// leetgo: 1.4.11
// https://leetcode.com/problems/search-in-rotated-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let lend = nums[0];
        let hend = nums[nums.len() - 1];

        if target == lend {
            return 0;
        } else if target == hend {
            return nums.len() as i32 - 1;
        }

        if lend > hend {
            // rotate-sorted
            let mut lo = 0;
            let mut hi = nums.len() - 1;
            while lo <= hi {
                let mid = lo + ((hi - lo) >> 1);

                if nums[mid] == target {
                    return mid as i32;
                }
                if nums[mid] >= nums[lo] {
                    // lo .. mid is sorted
                    if target >= nums[lo] && target < nums[mid] {
                        // target in lo .. mid
                        hi = mid - 1;
                    } else {
                        // target in mid .. hi
                        lo = mid + 1;
                    }
                } else {
                    // mid .. hi is sorted
                    if target > nums[mid] && target < nums[hi] {
                        // target in mid .. hi
                        lo = mid + 1;
                    } else {
                        // target in lo .. mid
                        hi = mid - 1;
                    }
                }
            }
            -1
        } else {
            // normal
            let mut lo = 0;
            let mut hi = nums.len() - 1;
            while lo <= hi {
                let mid = lo + ((hi - lo) >> 1);

                match nums[mid].cmp(&target) {
                    Ordering::Equal => return mid as i32,
                    Ordering::Less => lo = mid + 1,
                    Ordering::Greater => {
                        let nhi = hi.saturating_sub(1);
                        if hi == nhi {
                            break;
                        }
                        hi = nhi;
                    }
                }
            }
            -1
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::search(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
