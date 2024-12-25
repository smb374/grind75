# smb374/grind75

This is my own solution to grind 75.

Most of them are finished in Rust (except Linked List Cycle and Clone Graph).

I'm using [leetgo](https://github.com/j178/leetgo) to manage the questions and
local testing.

## Week 1

1. [Two Sum](/rust/src/0001.two-sum/question.md)
2. [Valid Parentheses](/rust/src/0020.valid-parentheses/question.md)
3. [Merge Two Sorted Lists](/rust/src/0021.merge-two-sorted-lists/question.md)
4. [Best Time to Buy and Sell Stock](/rust/src/0121.best-time-to-buy-and-sell-stock/question.md)
5. [Valid Palindrome](/rust/src/0125.valid-palindrome/question.md)
6. [Invert Binary Tree](/rust/src/0226.invert-binary-tree/question.md)
7. [Valid Anagram](/rust/src/0242.valid-anagram/question.md)
8. [Binary Search](/rust/src/0704.binary-search/question.md): Binary Search
9. [Flood Fill](/rust/src/0733.flood-fill/question.md): BFS
10. [Lowest Common Ancestor of a Binary Search Tree](/rust/src/0235.lowest-common-ancestor-of-a-binary-search-tree/question.md):
    DFS
11. [Balanced Binary Tree](/rust/src/0110.balanced-binary-tree/question.md):
    Recursive tree height building
12. [Linked List Cycle](/go/0141.linked-list-cycle/question.md)
13. [Implement Queue using Stacks](/rust/src/0232.implement-queue-using-stacks/question.md)

## Week 2

1. [First Bad Version](/rust/src/0278.first-bad-version/question.md): Binary Search
2. [Ransom Note](/rust/src/0383.ransom-note/question.md): Occurrence Table
3. [Climbing Stairs](/rust/src/0070.climbing-stairs/question.md):
   Small DP with 2 variables
4. [Longest Palindrome](/rust/src/0409.longest-palindrome/question.md):
   Occurrence Table
5. [Reverse Linked List](/rust/src/0206.reverse-linked-list/question.md)
6. [Majority Element](/rust/src/0169.majority-element/question.md):
   Boyer-Moore Majority Vote Algorithm
7. [Add Binary](/rust/src/0067.add-binary/question.md):
   Iterate 1-bit carry adder
8. [Diameter of Binary Tree](/rust/src/0543.diameter-of-binary-tree/question.md):
   Extra state during recursive tree height building
9. [Middle of the Linked List](/rust/src/0876.middle-of-the-linked-list/question.md):
   Fast-slow pointers
10. [Maximum Depth of Binary Tree](/rust/src/0104.maximum-depth-of-binary-tree/question.md):
    Recursive tree height building
11. [Contains Duplicate](/rust/src/0217.contains-duplicate/question.md):
    Visited set
12. [Maximum Subarray](/rust/src/0053.maximum-subarray/question.md):
    Kadane's algorithm

## Week 3

1. [Insert Interval](/rust/src/0057.insert-interval/question.md):
   Find partition points as ranges then splice the range with updated interval
2. [01 Matrix](/rust/src/0542.01-matrix/question.md): BFS
3. [K Closest Points to Origin](/rust/src/0973.k-closest-points-to-origin/question.md):
   Worst linear time selection algorithm
4. [Longest Substring Without Repeating Characters](/rust/src/0003.longest-substring-without-repeating-characters/question.md):
   Use a table to record start index for repeated element, update current start
   and max length in each iteration
5. [3Sum](/rust/src/0015.3sum/question.md): Sort then use two pointers, unlike 2Sum
6. [Binary Tree Level Order Traversal](/rust/src/0102.binary-tree-level-order-traversal/question.md):
   BFS
7. [Clone Graph](/cpp/0133.clone-graph/question.md):
   Use a table to memorize cloned nodes with BFS
8. [Evaluate Reverse Polish Notation](/rust/src/0150.evaluate-reverse-polish-notation/question.md):
   Stack

## Week 4

1. [Course Schedule](/rust/src/0207.course-schedule/question.md):
   Topological Sort using DFS with 3-state mark to detect cycle
