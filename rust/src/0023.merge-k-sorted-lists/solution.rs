// Created by Po-Yeh Chen at 2024/12/30 10:58
// leetgo: 1.4.11
// https://leetcode.com/problems/merge-k-sorted-lists/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::VecDeque;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut q: VecDeque<i32> = VecDeque::new();

        for list in lists {
            let mut curr = list;
            while let Some(node) = curr {
                let index = q.partition_point(|&x| x < node.val);
                q.insert(index, node.val);
                curr = node.next;
            }
        }

        let mut curr = None;

        while let Some(node) = q.pop_back() {
            curr = Some(Box::new(ListNode {
                val: node,
                next: curr,
            }));
        }

        curr
    }
}

// @lc code=end

fn main() -> Result<()> {
    let lists: Vec<LinkedList> = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::merge_k_lists(lists).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
