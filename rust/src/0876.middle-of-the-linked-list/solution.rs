// Created by Po-Yeh Chen at 2024/12/24 14:45
// leetgo: 1.4.11
// https://leetcode.com/problems/middle-of-the-linked-list/

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone();
        let mut slow = head;

        while fast.is_some() {
            // only step slow if fast.next is Some to catch odd length case
            if fast.as_ref().and_then(|n| n.next.as_ref()).is_some() {
                slow = slow.and_then(|n| n.next);
            }
            // fast step 2 nodes
            fast = fast.and_then(|n| n.next).and_then(|n| n.next.clone());
        }
        slow
    }
}

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::middle_node(head.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
