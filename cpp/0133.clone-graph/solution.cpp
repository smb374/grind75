// Created by Po-Yeh Chen at 2024/12/25 00:22
// leetgo: 1.4.11
// https://leetcode.com/problems/clone-graph/

#include "LC_IO.h"
#include <bits/stdc++.h>
using namespace std;

// Definition for a Node.
class Node {
  public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
// @lc code=begin

/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
*/

#include <queue>
#include <unordered_map>

class Solution {
  private:
    std::unordered_map<Node*, Node*> mem;

  public:
    Node* cloneGraph(Node* node) {
        if (!node) {
            return nullptr;
        }
        std::queue<Node*> queue;
        Node* copy = new Node(node->val, {});
        mem[node] = copy;

        queue.push(node);

        while (!queue.empty()) {
            Node* cur = queue.front();
            queue.pop();

            for (Node* n : cur->neighbors) {
                if (mem.find(n) == mem.end()) {
                    mem[n] = new Node(n->val, {});
                    queue.push(n);
                }
                mem[cur]->neighbors.push_back(mem[n]);
            }
        }

        return copy;
    }
};

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
int main() {
    ios_base::sync_with_stdio(false);
    stringstream out_stream;

    vector<vector<int>> edges;
    LeetCodeIO::scan(cin, edges);

    Solution* obj = new Solution();
    auto res = obj->cloneGraph(edges);
    LeetCodeIO::print(out_stream, res);
    cout << "\noutput: " << out_stream.rdbuf() << endl;

    delete obj;
    return 0;
}
