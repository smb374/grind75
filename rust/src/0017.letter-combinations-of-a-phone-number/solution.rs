// Created by Po-Yeh Chen at 2024/12/28 15:31
// leetgo: 1.4.11
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

fn digit_to_letters(c: u8) -> Vec<char> {
    match c as char {
        '2' => vec!['a', 'b', 'c'],
        '3' => vec!['d', 'e', 'f'],
        '4' => vec!['g', 'h', 'i'],
        '5' => vec!['j', 'k', 'l'],
        '6' => vec!['m', 'n', 'o'],
        '7' => vec!['p', 'q', 'r', 's'],
        '8' => vec!['t', 'u', 'v'],
        '9' => vec!['w', 'x', 'y', 'z'],
        _ => unreachable!(),
    }
}

fn routine(start: usize, path: &mut String, results: &mut Vec<String>, digits: &[u8]) {
    if start == digits.len() {
        results.push(path.clone());
    } else {
        for i in digit_to_letters(digits[start]) {
            path.push(i as char);
            routine(start + 1, path, results, digits);
            path.pop();
        }
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }
        let mut path = String::new();
        let mut results = Vec::new();

        routine(0, &mut path, &mut results, digits.as_bytes());

        results
    }
}

// @lc code=end

fn main() -> Result<()> {
    let digits: String = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::letter_combinations(digits).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
