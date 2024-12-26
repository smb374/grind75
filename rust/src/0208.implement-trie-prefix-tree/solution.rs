// Created by Po-Yeh Chen at 2024/12/25 13:13
// leetgo: 1.4.11
// https://leetcode.com/problems/implement-trie-prefix-tree/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
type Ptr<T> = Option<Box<T>>;

struct Node {
    mark: bool,
    children: [Ptr<Self>; 26],
}

impl Default for Node {
    fn default() -> Self {
        Self {
            mark: false,
            children: Default::default(),
        }
    }
}

#[derive(Default)]
struct Trie {
    root: Node,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let byt = word.as_bytes();
        let mut curr = &mut self.root;

        for &b in byt {
            let slot = b as usize - 0x61;
            curr = curr.children[slot].get_or_insert_with(|| Box::new(Node::default()));
        }
        curr.mark = true;
    }

    fn search(&self, word: String) -> bool {
        let byt = word.as_bytes();
        let mut curr = &self.root;

        for &b in byt {
            let slot = b as usize - 0x61;
            match &curr.children[slot] {
                Some(next) => {
                    curr = next;
                }
                None => return false,
            }
        }
        curr.mark
    }

    fn starts_with(&self, prefix: String) -> bool {
        let byt = prefix.as_bytes();
        let mut curr = &self.root;

        for &b in byt {
            let slot = b as usize - 0x61;
            match &curr.children[slot] {
                Some(next) => {
                    curr = next;
                }
                None => return false,
            }
        }
        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = Trie::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "insert" => {
                let method_params = split_array(&params[i])?;
                let word: String = deserialize(&method_params[0])?;
                obj.insert(word);
                output.push("null".to_string());
            }
            "search" => {
                let method_params = split_array(&params[i])?;
                let word: String = deserialize(&method_params[0])?;
                let ans: bool = obj.search(word).into();
                output.push(serialize(ans)?);
            }
            "startsWith" => {
                let method_params = split_array(&params[i])?;
                let prefix: String = deserialize(&method_params[0])?;
                let ans: bool = obj.starts_with(prefix).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
