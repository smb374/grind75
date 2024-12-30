// Created by Po-Yeh Chen at 2024/12/27 23:28
// leetgo: 1.4.11
// https://leetcode.com/problems/longest-palindromic-substring/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// expand out form i..j
fn expand(s: &[u8], i: usize, j: usize, left: &mut usize, right: &mut usize) {
    let mut i = i;
    let mut j = j;

    while i < s.len() && j < s.len() && s[i] == s[j] {
        if (j - i) > (*right - *left) {
            *left = i;
            *right = j;
        }
        i = i.wrapping_sub(1);
        j += 1;
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let byt = s.as_bytes();
        let mut left = 0;
        let mut right = 0;

        for i in 0..byt.len() {
            // Odd palindrome
            expand(byt, i, i, &mut left, &mut right);
            // Even palindrome
            expand(byt, i, i + 1, &mut left, &mut right);
        }

        s[left..=right].to_string()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: String = Solution::longest_palindrome(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
