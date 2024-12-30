// Created by Po-Yeh Chen at 2024/12/29 20:59
// leetgo: 1.4.11
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/

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

fn inorder_walk(node: Option<&Rc<RefCell<TreeNode>>>, k: i32) -> (Option<i32>, i32) {
    match node {
        None => (None, 0),
        Some(node) => {
            let nref = node.borrow();
            // Left side, k_left = k
            let lcnt = match inorder_walk(nref.left.as_ref(), k) {
                (Some(ans), _) => return (Some(ans), k),
                (None, c) if c == k - 1 => return (Some(nref.val), k),
                (None, c) => c,
            };
            // Right side, k_right = k - left_count - 1
            // |      | k    |
            // ---------^-----
            // | left |      |
            match inorder_walk(nref.right.as_ref(), k - lcnt - 1) {
                (Some(ans), _) => (Some(ans), k),
                (None, rcnt) => (None, rcnt + lcnt + 1),
            }
        }
    }
}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        inorder_walk(root.as_ref(), k).0.unwrap_or(-1)
        // if root.is_none() {
        //     return -1;
        // }
        // let k = k as usize - 1;
        // let mut v = Vec::new();
        // inorder_walk(root.as_ref(), &mut v);
        // if k >= v.len() {
        //     -1
        // } else {
        //     let (_, r, _) = v.select_nth_unstable(k as usize);
        //     *r
        // }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::kth_smallest(root.into(), k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
