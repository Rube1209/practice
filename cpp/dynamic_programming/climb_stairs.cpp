class Solution {
public:
    int climbStairs(int n) {
        if (n <= 2) return n;  // 如果只有1阶或2阶，直接返回n

        int first = 1;  // 到达第1阶的方法数
        int second = 2; // 到达第2阶的方法数
        int ways = 0;

        for (int i = 3; i <= n; ++i) {
            ways = first + second; // 当前阶的方式是前两阶方式的和
            first = second;        // 更新为前一阶
            second = ways;        // 更新为当前阶
        }

        return ways;
    }
};
