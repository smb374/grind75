// Created by Po-Yeh Chen at 2024/12/23 11:53
// leetgo: 1.4.11
// https://leetcode.com/problems/merge-two-sorted-lists/

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Simple recursive
        // match (list1, list2) {
        //     (l1, None) => l1,
        //     (None, l2) => l2,
        //     (Some(l1), Some(l2)) => {
        //         if l1.val < l2.val {
        //             Some(Box::new(ListNode {
        //                 val: l1.val,
        //                 next: Self::merge_two_lists(l1.next, Some(l2)),
        //             }))
        //         } else {
        //             Some(Box::new(ListNode {
        //                 val: l2.val,
        //                 next: Self::merge_two_lists(l2.next, Some(l1)),
        //             }))
        //         }
        //     }
        // }
        // Use mem swap to swap values, will mutate the list.
        let mut curr = &mut list1;
        // list2 is none -> return list1 directly
        while list2.is_some() {
            // curr is none || list2.val < curr.val -> swap curr and list2
            if curr.is_none() || list2.as_ref()?.val < curr.as_ref()?.val {
                std::mem::swap(curr, &mut list2);
            }
            // curr stepping to next
            curr = &mut curr.as_mut()?.next;
        }
        list1
    }
}

// @lc code=end

fn main() -> Result<()> {
    let list1: LinkedList = deserialize(&read_line()?)?;
    let list2: LinkedList = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::merge_two_lists(list1.into(), list2.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
