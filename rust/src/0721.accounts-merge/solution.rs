// Created by Po-Yeh Chen at 2024/12/26 14:16
// leetgo: 1.4.11
// https://leetcode.com/problems/accounts-merge/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin
use std::collections::HashMap;

// Union-find routines
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    // Path compression
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // Union by rank
    fn union(&mut self, x: usize, y: usize) {
        let mut a = self.find(x);
        let mut b = self.find(y);

        if a == b {
            return;
        }

        if self.rank[a] < self.rank[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // Use union-find to solve this
        // Disjoint Set for account index.
        let mut ds = DisjointSet::new(accounts.len());
        // Email -> account index in accounts
        let mut email_account = HashMap::new();
        // Storing merged accounts.
        let mut merged_accounts = HashMap::with_capacity(accounts.len());

        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                match email_account.get(email) {
                    Some(&j) => {
                        // union i-th account & j-th account as they shared the same email.
                        ds.union(i, j);
                    }
                    None => {
                        email_account.insert(email, i);
                    }
                }
            }
        }

        for (&email, &i) in email_account.iter() {
            // root account index.
            let root = ds.find(i);
            // construct account vec.
            merged_accounts
                .entry(root)
                .or_insert(vec![accounts[root][0].clone()]) // initialize with account name.
                .push(email.clone());
        }

        merged_accounts
            .into_values()
            .map(|mut account| {
                // sort the emails as requested
                account[1..].sort_unstable();
                account
            })
            .collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let accounts: Vec<Vec<String>> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<String>> = Solution::accounts_merge(accounts).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
