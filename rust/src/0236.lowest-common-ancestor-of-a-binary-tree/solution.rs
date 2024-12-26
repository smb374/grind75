// Created by Po-Yeh Chen at 2024/12/26 12:08
// leetgo: 1.4.11
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/

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

// DFS Routine
fn routine(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let nval = node.borrow().val;
        // Check if current node's val equals to p or q -> LCA is node itself.
        if nval == p || nval == q {
            Some(node)
        } else {
            // DFS left and right
            let left = routine(node.borrow().left.clone(), p, q);
            let right = routine(node.borrow().right.clone(), p, q);
            // Check result
            match (left, right) {
                // Both some => LCA is node itself
                (Some(_), Some(_)) => Some(node),
                // one of them is none -> node is parent of LCA (left or right)
                (left, None) => left,
                (None, right) => right,
            }
        }
    } else {
        None
    }
}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Thanks LeetCode :(
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        routine(root, p_val, q_val)
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let p: i32 = deserialize(&read_line()?)?;
    let q: i32 = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::lowest_common_ancestor(root.into(), p, q).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
