pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // 将 m 和 n 转换为 usize 以便于数组索引
        let m = m as usize;
        let n = n as usize;

        // 创建一个 m x n 的 dp 数组，初始化为 0
        let mut dp = vec![vec![0; n]; m];

        // 初始化第一行和第一列
        for i in 0..m {
            dp[i][0] = 1; // 第一列
        }
        for j in 0..n {
            dp[0][j] = 1; // 第一行
        }
        
        // 填充 dp 数组
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1]; // 状态转移方程
            }
        }

        // 返回右下角的路径数量
        dp[m - 1][n - 1]
    }
}

// 使用示例
fn main() {
    let solution = Solution;
    let m = 3;
    let n = 7;
    println!("Number of unique paths: {}", solution.unique_paths(m, n)); // 输出 28
}
