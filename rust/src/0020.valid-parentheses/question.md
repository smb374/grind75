# [20. Valid Parentheses][link] (Easy)

[link]: https://leetcode.com/problems/valid-parentheses/

Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`,
determine if the input string is valid.

An input string is valid if:

1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.
3. Every close bracket has a corresponding open bracket of the same type.

**Example 1:**

**Input:** s = "()"

**Output:** true

**Example 2:**

**Input:** s = "()\[\]{}"

**Output:** true

**Example 3:**

**Input:** s = "(\]"

**Output:** false

**Example 4:**

**Input:** s = "(\[\])"

**Output:** true

**Constraints:**

- `1 <= s.length <= 10⁴`
- `s` consists of parentheses only `'()[]{}'`.

## My Answer

For this one, use a stack to check if incoming right part closes stack top.

```rust
fn check_valid(lhs: Option<&char>, rhs: &char) -> bool {
    match lhs {
        Some(&'(') => *rhs == ')',
        Some(&'[') => *rhs == ']',
        Some(&'{') => *rhs == '}',
        _ => false,
    }
}

fn is_left(x: char) -> bool {
    x == '(' || x == '[' || x == '{'
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Use a stack to push.
        let mut stack: Vec<char> = Vec::with_capacity(s.len() >> 1);
        for c in s.chars() {
            if is_left(c) {
                stack.push(c);
            } else if check_valid(stack.last(), &c) {
                // Remember to use last to detect empty stack.
                stack.pop();
            } else {
                return false;
            }
        }
        stack.is_empty()
    }
}
```
