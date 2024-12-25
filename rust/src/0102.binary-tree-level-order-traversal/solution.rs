// Created by Po-Yeh Chen at 2024/12/25 00:09
// leetgo: 1.4.11
// https://leetcode.com/problems/binary-tree-level-order-traversal/

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        match root {
            None => vec![],
            Some(node) => {
                // Do a BFS on tree.
                let mut queue = VecDeque::new();
                let mut result: Vec<Vec<i32>> = Vec::new();

                queue.push_front((node, 0));

                while let Some((curr, level)) = queue.pop_back() {
                    match result.get_mut(level) {
                        Some(v) => {
                            v.push(curr.borrow().val);
                        }
                        None => {
                            result.push(vec![curr.borrow().val]);
                        }
                    }
                    if let Some(left) = curr.borrow().left.as_ref() {
                        queue.push_front((Rc::clone(left), level + 1));
                    }
                    if let Some(right) = curr.borrow().right.as_ref() {
                        queue.push_front((Rc::clone(right), level + 1));
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
    let ans: Vec<Vec<i32>> = Solution::level_order(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
