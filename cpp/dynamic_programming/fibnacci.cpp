class Solution {
public:
    int fib(int n) {
        if (n == 0) return 0;
        if (n == 1) return 1;

        int a = 0, b = 1;  // F(0) 和 F(1)
        for (int i = 2; i <= n; ++i) {
            int c = a + b;  // 计算当前 F(n)
            a = b;          // 更新 F(n-2)
            b = c;          // 更新 F(n-1)
        }
        return b;  // 返回 F(n)
    }
};
