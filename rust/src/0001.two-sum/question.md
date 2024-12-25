# [1. Two Sum][link] (Easy)

[link]: https://leetcode.com/problems/two-sum/

Given an array of integers `nums` and an integer `target`, return indices of the two numbers such
that they add up to `target`.

You may assume that each input would have **exactly one solution**, and you may not use the same
element twice.

You can return the answer in any order.

**Example 1:**

```
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

**Example 2:**

```
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

**Example 3:**

```
Input: nums = [3,3], target = 6
Output: [0,1]
```

**Constraints:**

- `2 <= nums.length <= 10⁴`
- `-10⁹ <= nums[i] <= 10⁹`
- `-10⁹ <= target <= 10⁹`
- **Only one valid answer exists.**

**Follow-up:** Can you come up with an algorithm that is less than `O(n²)` time complexity?

## My Answer

We can use a `HashMap<i32, usize>` to store the visited number & its index.
When we hit 1 that's in the table, return the index.

Time complexity: $`O(n)`$

```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // iterate nums and momorize visited valu and index with memo
        // if we found (target - v) in memo, thats the answer.
        let mut result = vec![-1, -1];
        let mut memo = HashMap::with_capacity(nums.len());
        for (i, &v) in nums.iter().enumerate() {
            let diff = target - v;
            if let Some(&j) = memo.get(&diff) {
                result[0] = j as i32;
                result[1] = i as i32;
                break;
            }
            memo.insert(v, i);
        }
        result
    }
}
```
