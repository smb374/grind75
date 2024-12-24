// Created by Po-Yeh Chen at 2024/12/23 19:33
// leetgo: 1.4.11
// https://leetcode.com/problems/balanced-binary-tree/

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
use std::rc::Rc;

// Use optional return to detect unbalanced subtree.
// try will create an avalanche effect s.t. parent call will also return None.
fn tree_height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
    match root {
        Some(node) => {
            let lheight = tree_height(&node.borrow().left)?;
            let rheight = tree_height(&node.borrow().right)?;

            let diff = (lheight - rheight).abs();
            if diff <= 1 {
                Some(lheight.max(rheight) + 1)
            } else {
                None
            }
        }
        None => Some(0),
    }
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        tree_height(&root).is_some()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_balanced(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
