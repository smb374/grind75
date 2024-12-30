// Created by Po-Yeh Chen at 2024/12/29 21:53
// leetgo: 1.4.11
// https://leetcode.com/problems/word-ladder/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashSet;

fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| x.ne(y) as usize).sum()
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // Word in word_list as nodes, exists edge (p, q) if the hamming distance of node p and node
        // q is one and q is in the word_list.
        //
        // On such graph, use Bidirectional BFS to search the distance between begin_word and
        // end_word.
        let mut words: Vec<&str> = word_list.iter().map(|x| x.as_str()).collect();
        if !words.contains(&end_word.as_str()) {
            return 0;
        }
        let mut head: HashSet<&str> = HashSet::with_capacity(word_list.len() + 1);
        let mut tail: HashSet<&str> = HashSet::with_capacity(word_list.len() + 1);
        let mut next_level: HashSet<&str> = HashSet::with_capacity(word_list.len() + 1);
        head.insert(&begin_word);
        tail.insert(&end_word);
        // Since begin and end is already explored, hence level = 1
        let mut level = 1;

        while !head.is_empty() && !tail.is_empty() {
            if head.len() > tail.len() {
                std::mem::swap(&mut head, &mut tail);
            }

            for p in head.drain() {
                for q in tail.iter() {
                    if hamming_distance(p.as_bytes(), q.as_bytes()) == 1 {
                        return level + 1;
                    }
                }

                // visited means we won't touch it again -> using swap_remove is the same as using
                // a HashSet to record visited words.
                let mut idx = 0;
                while idx < words.len() {
                    if hamming_distance(p.as_bytes(), words[idx].as_bytes()) != 1 {
                        idx += 1;
                        continue;
                    }

                    let q = words.swap_remove(idx);
                    next_level.insert(q);
                }
            }

            std::mem::swap(&mut head, &mut next_level);
            level += 1;
        }
        0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let begin_word: String = deserialize(&read_line()?)?;
    let end_word: String = deserialize(&read_line()?)?;
    let word_list: Vec<String> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::ladder_length(begin_word, end_word, word_list).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
