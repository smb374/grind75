// Created by Po-Yeh Chen at 2024/12/24 11:38
// leetgo: 1.4.11
// https://leetcode.com/problems/reverse-linked-list/

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
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(head) => {
                let mut curr = Some(head);
                let mut result = None;

                while let Some(mut node) = curr {
                    // Save next ptr
                    let tmp = node.next;
                    // Make node next to current result
                    node.next = result;
                    // Update result to node.
                    result = Some(node);
                    // curr step to saved next
                    curr = tmp;
                }

                result
            }
            None => None,
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::reverse_list(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
