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
    DFS on one child by BST's property.
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
   Find partition points for new interval's start (`|x| x[1] < new_interval[0]`)
   and end (`|x| x[0] <= new_interval[1]`), then splice the range with merged interval
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
2. [Implement Trie (Prefix Tree)](/rust/src/0208.implement-trie-prefix-tree/question.md):
   Use `[Option<Box<Node>>; 26]` as children as only lower case alphas in input.
3. [Coin Change](/rust/src/0322.coin-change/question.md):
   Use 1-D DP based on each coin value (`mem[i] = mem[i].min(mem[i - c] + 1)`).
4. [Product of Array Except Self](/rust/src/0238.product-of-array-except-self/question.md):
   - Use a vec of tuples to record prefix & suffix product of index,
     then `.map(|(x, y)| x * y).collect()`
   - Use 2 variables to track current prefix & suffix product.
5. [Min Stack](/rust/src/0155.min-stack/question.md):
   Use another stack `previous_min` to record minimums for each push operation.
6. [Validate Binary Search Tree](/rust/src/0098.validate-binary-search-tree/question.md):
   Use in-order traversal and record visited values in order then
   validate the record is strict ascending with `.windows(2).all(|w| w[0] < w[1])`.
7. [Number of Islands](/rust/src/0200.number-of-islands/question.md):
   Use BFS like routine to overwrite neighboring 1s to 0s on encountered 1s,
   update count before routine start.
8. [Rotting Oranges](/rust/src/0994.rotting-oranges/question.md):
   Use multi-source BFS and visited table to prevent overstepping,
   update max count during BFS, then check if there are remaining 1s.

## Week 5

1. [Search in Rotated Sorted Array](/rust/src/0033.search-in-rotated-sorted-array/question.md):
   Rotated Sorted:
   `arr[0] < arr[peak] && arr[len(arr) - 1] < arr[peak] && arr[len(arr - 1)] < arr[0]`
2. [Combination Sum](/rust/src/0039.combination-sum/question.md):
   Backtracking
3. [Permutations](/rust/src/0046.permutations/question.md):
   Backtracking
4. [Merge Intervals](/rust/src/0056.merge-intervals/question.md):
   Sort the intervals the push the first interval into result vec.
   For the rest intervals `i`, merge if `i.start < result[idx].end`,
   or push interval then increase idx.
5. [Lowest Common Ancestor of a Binary Tree](/rust/src/0236.lowest-common-ancestor-of-a-binary-tree/question.md):
   Unlike the [BST](/rust/src/0235.lowest-common-ancestor-of-a-binary-search-tree/question.md)
   one, we need to visit both children in the DFS routine in recursive fashion than
   direct return.
6. [Time Based Key-Value Store](/rust/src/0981.time-based-key-value-store/question.md):
   Use a hash map to store a vector of tuples `(timestamp, item)`.
   - set: Push the tuple
   - get: binary search the item where its timestamp <= argument's timestamp.
7. [Accounts Merge](/rust/src/0721.accounts-merge/question.md):
   Use union-find by rank to union the accounts that shares the same email
   (use email to account index table for lookup), then merge the emails with
   the same root to the same account, then finally sort the emails before return.

8. [Sort Colors](/rust/src/0075.sort-colors/question.md):
   - 2-pass counting sort: count 0s and 1s in first pass, then sort nums.
   - 1-pass 3-pointers: Use `lo` for upper limit of 0s, `hi` for lower limit of 2s,
     and `i` to step. `swap(i, lo); i++; lo++` if `nums[i] == 0`,
     `swap(i, hi); hi--` if `nums[i] == 2`, `i++` otherwise while `i <= hi`.

## Week 6

1. [Word Break](/rust/src/0139.word-break/question.md):
   DFS with momoization on index,
   which indicates we can segment the string to a word in words starting from index.
2. [Partition Equal Subset Sum](/rust/src/0416.partition-equal-subset-sum/question.md):
   0/1 Knapsack DP,
   - Add the array and check if total is even, odd -> we can't divide.
   - Set `target = total >> 1`
   - Initialize DP array `mem` with false and size of `target + 1`,
     set `mem[0] = true`.
   - For each number `n`, we iterate `i` from `target` down to `n` with
     `mem[i] = mem[i] || mem[i - n]`,
     which means taking `(i - n)` or not (hence 0/1 Knapsack).
   - If `mem[target] == true` in the loop, stop and return true,
     otherwise return false after we iterated all numbers.
3. [String to Integer (atoi)](/rust/src/0008.string-to-integer-atoi/question.md):
   Check the sign and the starting index with `s[0]`, then take iterate
   all the digits from start and update the accumulator, finally apply the sign.
4. [Spiral Matrix](/rust/src/0054.spiral-matrix/question.md):
   Maintain a visited matrix and perform spiral walk with `RDLU` direction.
5. [Subsets](/rust/src/0078.subsets/question.md):
   DFS backtracking with 2 branch: take current number or not.
6. [Binary Tree Right Side View](/rust/src/0199.binary-tree-right-side-view/question.md):
   BFS in a level fashion and push right instead of left first to the queue.
7. [Longest Palindromic Substring](/rust/src/0005.longest-palindromic-substring/question.md):
   For each index, expand and record the largest start and end index of a palindrome.
   Remember to check both even (start: `i` & `i+1`) and odd (start: `i` & `i`) palindromes.
8. [Unique Paths](/rust/src/0062.unique-paths/question.md):
   2D DP on the `m x n` grid:
   - If one of `m` or `n` is 0, return 0
   - Initialize a `m x n` 2D DP array `path` with 0
   - For each cell
     - If `i==0 || j==0`, `path[i][j] = 1` as there's only 1 path to `(i, j)`
     - Else `path[i][j] = path[i][j-1] + path[i-1][j]`
   - Return `path[m-1][n-1]`
9. [Construct Binary Tree from Preorder and Inorder Traversal](/rust/src/0105.construct-binary-tree-from-preorder-and-inorder-traversal/question.md):
   DFS explore the pre-order walk array
   `dfs(preorder, inorder_index, pre_idx, in_idx, size)`:
   - In each DFS iteration, get `val = preorder[pre_idx]`, `idx = inorder_index[val]`,
     `left_size = idx-in_idx`.
   - Explore left to get left sub tree:
     `left = dfs(preorder, inorder_index, pre_idx+1, in_idx, left_size)`
   - Explore right to get right sub tree:
     `right = dfs(preorder, inorder_index, pre_idx+1+left_size, idx+1, size-1-left_size)`
   - Return a new node with `Node {val, left, right}`

## Week 7

1. [Container With Most Water](/rust/src/0011.container-with-most-water/question.md)
2. [Letter Combinations of a Phone Number](/rust/src/0017.letter-combinations-of-a-phone-number/question.md)
3. [Word Search](/rust/src/0079.word-search/question.md)
4. [Find All Anagrams in a String](/rust/src/0438.find-all-anagrams-in-a-string/question.md)
5. [Minimum Height Trees](/rust/src/0310.minimum-height-trees/question.md)
6. [Task Scheduler](/rust/src/0621.task-scheduler/question.md)
7. [LRU Cache](/rust/src/0146.lru-cache/question.md)

## Week 8

1. [Kth Smallest Element in a BST](/rust/src/0230.kth-smallest-element-in-a-bst/question.md)
2. [Minimum Window Substring](/cpp/0076.minimum-window-substring/question.md)
3. [Serialize and Deserialize Binary Tree](/rust/src/0297.serialize-and-deserialize-binary-tree/question.md)
4. [Trapping Rain Water](/rust/src/0042.trapping-rain-water/question.md)
5. [Find Median from Data Stream](/rust/src/0295.find-median-from-data-stream/question.md)
6. [Word Ladder](/rust/src/0127.word-ladder/question.md)
7. [Basic Calculator](/rust/src/0224.basic-calculator/question.md)
8. [Maximum Profit in Job Scheduling](/rust/src/1235.maximum-profit-in-job-scheduling/question.md)
9. [Merge k Sorted Lists](/rust/src/0023.merge-k-sorted-lists/question.md)
10. [Largest Rectangle in Histogram](/rust/src/0084.largest-rectangle-in-histogram/question.md)
