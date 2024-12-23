// Created by Po-Yeh Chen at 2024/12/23 15:48
// leetgo: 1.4.11
// https://leetcode.com/problems/invert-binary-tree/

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

trait Swap {
    fn swap(&mut self);
    fn swap_all(&mut self);
}

impl Swap for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    fn swap_all(&mut self) {
        if let Some(l) = self.left.as_mut() {
            l.borrow_mut().swap_all();
        }
        if let Some(r) = self.right.as_mut() {
            r.borrow_mut().swap_all();
        }
        self.swap();
    }
}

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Simple Recursive
        // if let Some(node) = root {
        //     let left = Self::invert_tree((*node).borrow().left.clone());
        //     let right = Self::invert_tree((*node).borrow().right.clone());
        //     (*node).borrow_mut().right = left;
        //     (*node).borrow_mut().left = right;
        //     Some(node)
        // } else {
        //     None
        // }
        if let Some(node) = root.as_mut() {
            node.borrow_mut().swap_all();
        }
        root
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::invert_tree(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
