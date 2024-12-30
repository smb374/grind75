// Created by Po-Yeh Chen at 2024/12/29 20:43
// leetgo: 1.4.11
// https://leetcode.com/problems/minimum-window-substring/

#include "LC_IO.h"
#include <bits/stdc++.h>
using namespace std;

// @lc code=begin

class Solution {
  public:
    string minWindow(string& original, string& check) {
        std::unordered_map<char, int> check_count;
        for (char c : check) {
            ++check_count[c];
        }

        std::unordered_map<char, int> window_count;
        int m = original.size();
        int window = -1;
        int length = m + 1;
        int satisfied = 0;
        int required = check_count.size();
        int i = 0;

        for (int j = 0; j < m; j++) {
            if (check_count.count(original[j])) {
                window_count[original[j]]++;
                if (window_count[original[j]] == check_count[original[j]]) {
                    satisfied++;
                }
            }
            while (satisfied == required) {
                if (j - i + 1 < length ||
                    (j - i + 1 == length &&
                     original.compare(i, length, original, window, length) <
                         0)) {
                    window = i;
                    length = j - i + 1;
                }
                // delete only characters from check
                if (check_count.count(original[i])) {
                    window_count[original[i]]--;
                    if (window_count[original[i]] < check_count[original[i]]) {
                        // removing original[i] makes window dissatisfied
                        satisfied--;
                    }
                }
                i++;
            }
        }
        return window >= 0 ? original.substr(window, length) : "";
    }
};

// @lc code=end

int main() {
    ios_base::sync_with_stdio(false);
    stringstream out_stream;

    string s;
    LeetCodeIO::scan(cin, s);
    string t;
    LeetCodeIO::scan(cin, t);

    Solution* obj = new Solution();
    auto res = obj->minWindow(s, t);
    LeetCodeIO::print(out_stream, res);
    cout << "\noutput: " << out_stream.rdbuf() << endl;

    delete obj;
    return 0;
}
