// Created by Po-Yeh Chen at 2024/12/27 23:18
// leetgo: 1.4.11
// https://leetcode.com/problems/binary-tree-right-side-view/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        // BFS, push right first.
        // In each loop (level), push back's val to result.
        // Note that the loop need to consume 1 level at a time.
        match root.as_ref() {
            None => Vec::new(),
            Some(node) => {
                let mut result = Vec::new();
                queue.push_front(node.clone());

                while !queue.is_empty() {
                    let n = queue.len();
                    let val = queue.back().map(|n| n.borrow().val).unwrap();
                    result.push(val);

                    for _ in 0..n {
                        if let Some(node) = queue.pop_back() {
                            if let Some(right) = node.borrow().right.as_ref() {
                                queue.push_front(right.clone());
                            }
                            if let Some(left) = node.borrow().left.as_ref() {
                                queue.push_front(left.clone());
                            }
                        }
                    }
                }

                result
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::right_side_view(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
