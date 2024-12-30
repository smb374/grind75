// Created by Po-Yeh Chen at 2024/12/29 20:19
// leetgo: 1.4.11
// https://leetcode.com/problems/minimum-window-substring/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if &s == &t {
            return t;
        } else if t.len() > s.len() {
            return String::new();
        }
        let s_byt = s.as_bytes();
        let t_byt = t.as_bytes();
        let t_freq: HashMap<u8, i32> = t_byt.iter().fold(HashMap::new(), |mut m, &x| {
            m.entry(x).and_modify(|v| *v += 1).or_insert(1);
            m
        });

        let required = t_freq.len(); // Note: frquency satisfied count
        let mut w_freq: HashMap<u8, i32> = HashMap::new();
        let mut start = None;
        let mut len = s.len() + 1;
        let mut satisfied = 0;
        let mut i = 0;

        for j in 0..s_byt.len() {
            if let Some(&tval) = t_freq.get(&s_byt[j]) {
                let wval = *w_freq.entry(s_byt[j]).and_modify(|v| *v += 1).or_insert(1);
                if wval == tval {
                    satisfied += 1;
                }
            }
            while satisfied == required {
                let wlen = j - i + 1;
                if wlen < len {
                    start = Some(i);
                    len = wlen;
                } else if let Some(idx) = start {
                    if wlen == len && &s[i..j] < &s[idx..idx + len] {
                        start = Some(i);
                        len = wlen;
                    }
                }
                if let Some(&tval) = t_freq.get(&s_byt[i]) {
                    let wval = *w_freq.entry(s_byt[i]).and_modify(|v| *v -= 1).or_default();
                    if wval < tval {
                        satisfied -= 1;
                    }
                }
                i += 1;
            }
        }

        if let Some(idx) = start {
            s[idx..idx + len].to_string()
        } else {
            String::new()
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: String = Solution::min_window(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
