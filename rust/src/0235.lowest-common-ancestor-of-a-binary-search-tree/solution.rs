// Created by Po-Yeh Chen at 2024/12/23 19:13
// leetgo: 1.4.11
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(root) => match (p, q) {
                (Some(p), Some(q)) => {
                    let vmin = std::cmp::min(p.borrow().val, q.borrow().val);
                    let vmax = std::cmp::max(p.borrow().val, q.borrow().val);

                    let mut stack = vec![root.clone()];
                    while let Some(node) = stack.pop() {
                        let nval = node.borrow().val;
                        // Case 1. in the middle, return.
                        if nval >= vmin && nval <= vmax {
                            return Some(node);
                        }

                        // Case 2. node val is larger than max(p, q)
                        // push left child if Some because of BST
                        if nval > vmax {
                            if let Some(nleft) = node.borrow().left.as_ref() {
                                stack.push(Rc::clone(nleft));
                            }
                        }

                        // Case 3. node val is smaller than min(p, q)
                        // push right child if Some because of BST
                        if nval < vmin {
                            if let Some(nright) = node.borrow().right.as_ref() {
                                stack.push(Rc::clone(nright));
                            }
                        }
                    }
                    Some(root)
                }
                _ => Some(root),
            },
            None => None,
        }
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let p: BinaryTree = deserialize(&read_line()?)?;
    let q: BinaryTree = deserialize(&read_line()?)?;
    let ans: BinaryTree = Solution::lowest_common_ancestor(root.into(), p.into(), q.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
