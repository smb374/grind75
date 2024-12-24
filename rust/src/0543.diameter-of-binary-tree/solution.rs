// Created by Po-Yeh Chen at 2024/12/24 14:25
// leetgo: 1.4.11
// https://leetcode.com/problems/diameter-of-binary-tree/

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

fn diameter_step(root: Option<Rc<RefCell<TreeNode>>>, diameter: i32) -> (i32, i32) {
    match root {
        None => (0, diameter),
        Some(node) => {
            let (lh, d1) = diameter_step(node.borrow().left.clone(), diameter);
            let (rh, d2) = diameter_step(node.borrow().right.clone(), d1);

            // calculate current max diameter, which must be >= left.height + right.height of
            // current node.
            let dr = d2.max(lh + rh);

            // return (height, current diameter)
            (1 + lh.max(rh), dr)
        }
    }
}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        diameter_step(root, 0).1
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: i32 = Solution::diameter_of_binary_tree(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
