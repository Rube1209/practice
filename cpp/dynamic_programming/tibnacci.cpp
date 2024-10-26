class Solution {
public:
    int tribonacci(int n) {
        if (n == 0) return 0;  // T(0) = 0
        if (n == 1 || n == 2) return 1;  // T(1) = 1, T(2) = 1
        
        int a = 0, b = 1, c = 1, d = 0; // T(0), T(1), T(2), T(n)

        // 从 T(3) 开始计算
        for (int i = 3; i <= n; i++) {
            d = a + b + c; // 计算 T(n) = T(n-1) + T(n-2) + T(n-3)
            a = b; // 更新 T(n-3)
            b = c; // 更新 T(n-2)
            c = d; // 更新 T(n-1)
        }
        
        return c; // 返回 T(n)
    }
};
