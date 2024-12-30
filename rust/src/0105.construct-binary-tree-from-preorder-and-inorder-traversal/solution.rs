// Created by Po-Yeh Chen at 2024/12/27 23:00
// leetgo: 1.4.11
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

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
use std::collections::HashMap;
use std::rc::Rc;

fn routine(
    preorder: &[i32],
    inorder_index: &HashMap<i32, usize>,
    pre_idx: usize,
    in_idx: usize,
    size: isize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if size <= 0 {
        None
    } else {
        // current node value
        let val = preorder[pre_idx];
        // current node's index in inorder walk
        let idx = inorder_index[&val];
        // left child size
        let left_size = idx as isize - in_idx as isize;
        // preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
        // at start:
        // val = 3, idx = 1, left_size = 1 - 0 = 1, left nodes = [3], right nodes = [20, 15, 7]
        // left = routine(preorder, inorder_index, 1, 0, 1)
        // right = routine(preorder, inorder_index, 2, 1, 3)

        let left = routine(preorder, inorder_index, pre_idx + 1, in_idx, left_size);
        let right = routine(
            preorder,
            inorder_index,
            (pre_idx as isize + 1 + left_size) as usize,
            idx + 1,
            size - 1 - left_size,
        );

        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_index: HashMap<i32, usize> = HashMap::new();

        for (i, &e) in inorder.iter().enumerate() {
            inorder_index.insert(e, i);
        }

        routine(&preorder, &inorder_index, 0, 0, preorder.len() as isize)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let preorder: Vec<i32> = deserialize(&read_line()?)?;
    let inorder: Vec<i32> = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::build_tree(preorder, inorder).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
