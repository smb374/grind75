// Created by Po-Yeh Chen at 2024/12/29 21:29
// leetgo: 1.4.11
// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/

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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut path = Vec::new();
        Self::serialize_walk(root.as_ref(), &mut path);
        path.join(" ")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data.split(' ');
        Self::deserialize_walk(&mut iter)
    }

    fn serialize_walk(node: Option<&Rc<RefCell<TreeNode>>>, path: &mut Vec<String>) {
        match node {
            None => {
                path.push("x".to_string());
            }
            Some(node) => {
                let nref = node.borrow();
                path.push(format!("{}", nref.val));

                Self::serialize_walk(nref.left.as_ref(), path);
                Self::serialize_walk(nref.right.as_ref(), path);
            }
        }
    }

    fn deserialize_walk(path: &mut (dyn Iterator<Item = &str>)) -> Option<Rc<RefCell<TreeNode>>> {
        match path.next() {
            None => None,
            Some(x) if x == "x" => None,
            Some(x) => {
                let Ok(val) = i32::from_str_radix(x, 10) else {
                    return None;
                };
                let left = Self::deserialize_walk(path);
                let right = Self::deserialize_walk(path);

                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
        }
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let root: BinaryTree = deserialize(&read_line()?)?;
    let ans: String = Solution::codec(root.into()).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
