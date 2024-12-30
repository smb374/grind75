// Created by Po-Yeh Chen at 2024/12/30 00:19
// leetgo: 1.4.11
// https://leetcode.com/problems/find-median-from-data-stream/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    lo: BinaryHeap<i32>,          // MAX HEAP
    hi: BinaryHeap<Reverse<i32>>, // MIN HEAP
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            lo: BinaryHeap::with_capacity(128),
            hi: BinaryHeap::with_capacity(128),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.lo.push(num);

        if self.lo.len() != 1 {
            self.balance();
        }
    }

    fn balance(&mut self) {
        // When size not balanced or top not balanced
        while let Some(&lo_top) = self.lo.peek() {
            if self.lo.len() > self.hi.len() + 1
                || self
                    .hi
                    .peek()
                    .map(|&Reverse(hi_top)| lo_top > hi_top)
                    .unwrap_or(false)
            {
                self.lo.pop();
                self.hi.push(Reverse(lo_top));
            } else {
                break;
            }
        }

        // Re adjust based on height.
        while self.hi.len() > self.lo.len() {
            self.lo.push(self.hi.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        // let size = self.data.len();
        // let idx = size >> 1;
        //
        // if size & 1 == 1 {
        //     self.data[idx] as f64
        // } else {
        //     (self.data[idx - 1] as f64 + self.data[idx] as f64) / 2.0
        // }
        if self.lo.len() == self.hi.len() {
            let mut med = 0.0;
            if let Some(&lo_top) = self.lo.peek() {
                med += lo_top as f64;
            }
            if let Some(&Reverse(hi_top)) = self.hi.peek() {
                med += hi_top as f64;
            }
            med / 2.0
        } else {
            self.lo.peek().map(|&x| x as f64).unwrap_or(0.0)
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = MedianFinder::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "addNum" => {
                let method_params = split_array(&params[i])?;
                let num: i32 = deserialize(&method_params[0])?;
                obj.add_num(num);
                output.push("null".to_string());
            }
            "findMedian" => {
                let ans: f64 = obj.find_median().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
