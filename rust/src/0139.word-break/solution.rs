// Created by Po-Yeh Chen at 2024/12/26 17:14
// leetgo: 1.4.11
// https://leetcode.com/problems/word-break/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;

fn routine<'a>(s: &'a str, start: usize, mem: &mut HashMap<usize, bool>, words: &[String]) -> bool {
    if start == s.len() {
        true
    } else if let Some(&v) = mem.get(&start) {
        v
    } else {
        let mut ans = false;

        for w in words.iter() {
            let end = start + w.len();
            if end > s.len() {
                continue;
            }
            // slice & test seg == w
            let seg = &s[start..end];
            if seg == w {
                // Check result for end..s.len()
                if routine(s, end, mem, words) {
                    ans = true;
                    break;
                }
            }
        }

        // Update mem result for start
        mem.insert(start, ans);
        ans
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // Backtracking

        // memoization map, mem.get(&start_index)
        // None => not visited
        // Some(false) => start_index bad
        // Some(true) => start_index good
        let mut mem: HashMap<usize, bool> = HashMap::new();
        routine(s.as_str(), 0, &mut mem, &word_dict)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let word_dict: Vec<String> = deserialize(&read_line()?)?;
    let ans: bool = Solution::word_break(s, word_dict).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
