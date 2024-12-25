# [21. Merge Two Sorted Lists][link] (Easy)

[link]: https://leetcode.com/problems/merge-two-sorted-lists/

You are given the heads of two sorted linked lists `list1` and `list2`.

Merge the two lists into one **sorted** list. The list should be made by splicing together the nodes
of the first two lists.

Return the head of the merged linked list.

**Example 1:**

![](https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg)

```
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
```

**Example 2:**

```
Input: list1 = [], list2 = []
Output: []
```

**Example 3:**

```
Input: list1 = [], list2 = [0]
Output: [0]
```

**Constraints:**

- The number of nodes in both lists is in the range `[0, 50]`.
- `-100 <= Node.val <= 100`
- Both `list1` and `list2` are sorted in **non-decreasing** order.

## My Answer

1. Using `std::mem::swap` to swap the content of `curr` and `&mut list2`.
   Only the content is swapped, it doesn't change the point value,
   such that we can merge the list conveniently without extra allocation.

    ```rust
    impl Solution {
        pub fn merge_two_lists(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            // Use mem swap to swap values, will mutate the list.
            let mut curr = &mut list1;
            // list2 is none -> return list1 directly
            while list2.is_some() {
                // curr is none || list2.val < curr.val -> swap curr and list2
                if curr.is_none() || list2.as_ref()?.val < curr.as_ref()?.val {
                    std::mem::swap(curr, &mut list2);
                }
                // curr stepping to next
                curr = &mut curr.as_mut()?.next;
            }
            list1
        }
    }
    ```

2. Simple recursion based solution.

    ```rust
    impl Solution {
        pub fn merge_two_lists(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            // Simple recursive
            match (list1, list2) {
                (l1, None) => l1,
                (None, l2) => l2,
                (Some(l1), Some(l2)) => {
                    if l1.val < l2.val {
                        Some(Box::new(ListNode {
                            val: l1.val,
                            next: Self::merge_two_lists(l1.next, Some(l2)),
                        }))
                    } else {
                        Some(Box::new(ListNode {
                            val: l2.val,
                            next: Self::merge_two_lists(l2.next, Some(l1)),
                        }))
                    }
                }
            }
        }
    }
    ```
