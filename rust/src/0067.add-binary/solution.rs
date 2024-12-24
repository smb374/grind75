// Created by Po-Yeh Chen at 2024/12/24 14:00
// leetgo: 1.4.11
// https://leetcode.com/problems/add-binary/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

use std::iter::repeat;
fn add_bin_digit(a: char, b: char, carry: bool) -> (char, bool) {
    match (a, b) {
        ('0', '0') => {
            if carry {
                ('1', false)
            } else {
                ('0', false)
            }
        }
        ('0', '1') | ('1', '0') => {
            if carry {
                ('0', true)
            } else {
                ('1', false)
            }
        }
        ('1', '1') => {
            if carry {
                ('1', true)
            } else {
                ('0', true)
            }
        }
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (small, big) = if a.len() > b.len() { (b, a) } else { (a, b) };

        let (mut res, carry) = small
            .chars()
            .rev()
            .chain(repeat('0'))
            .zip(big.chars().rev())
            .fold((String::new(), false), |(mut acc, carry), (c1, c2)| {
                let (r, nc) = add_bin_digit(c1, c2, carry);
                acc.push(r);
                (acc, nc)
            });

        if carry {
            res.push('1');
        }

        res.chars().rev().collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let a: String = deserialize(&read_line()?)?;
    let b: String = deserialize(&read_line()?)?;
    let ans: String = Solution::add_binary(a, b).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
