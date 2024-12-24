// Created by Po-Yeh Chen at 2024/12/23 19:53
// leetgo: 1.4.11
// https://leetcode.com/problems/linked-list-cycle/

package main

import (
	"bufio"
	"fmt"
	"os"

	. "github.com/j178/leetgo/testutils/go"
)

// @lc code=begin
import "math"

func hasCycle(head *ListNode) bool {
	if head == nil {
		return false
	}

	cur := head
	for cur != nil {
		if cur.Val == math.MaxInt {
			return true
		}
		cur.Val = math.MaxInt
		cur = cur.Next
	}

	return false
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
func main() {
	stdin := bufio.NewReader(os.Stdin)
	head := Deserialize[*ListNode](ReadLine(stdin))
	_ = Deserialize[int](ReadLine(stdin))
	ans := hasCycle(head)

	fmt.Println("\noutput:", Serialize(ans))
}
