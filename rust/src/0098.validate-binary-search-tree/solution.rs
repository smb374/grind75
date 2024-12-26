// Created by Po-Yeh Chen at 2024/12/25 18:12
// leetgo: 1.4.11
// https://leetcode.com/problems/validate-binary-search-tree/

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

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // DFS
        // traverse(root.as_ref(), i64::MIN, i64::MAX)

        // in order walk
        let mut record = Vec::new();
        inorder_walk(root.as_ref(), &mut record);

        if record.len() < 2 {
            true
        } else {
            record.windows(2).all(|w| w[0] < w[1])
        }
    }
}

// fn traverse(node: Option<&Rc<RefCell<TreeNode>>>, less: i64, great: i64) -> bool {
//     match node {
//         None => true, // leaf is always valid.
//         Some(node) => {
//             let nval = node.borrow().val as i64;
//
//             if nval <= less || nval >= great {
//                 false
//             } else {
//                 // less < left.val < nval
//                 // nval < right.val < great
//                 traverse(node.borrow().left.as_ref(), less, nval)
//                     && traverse(node.borrow().right.as_ref(), nval, great)
//             }
//         }
//     }
// }

fn inorder_walk(node: Option<&Rc<RefCell<TreeNode>>>, record: &mut Vec<i32>) {
    let Some(node) = node.map(|n| n.borrow()) else {
        return;
    };

    inorder_walk(node.left.as_ref(), record);
    record.push(node.val);
    inorder_walk(node.right.as_ref(), record);
}

// @lc code=end

fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_valid_bst(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
