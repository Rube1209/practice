pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0; // 检查输入是否为空
        }

        let m = grid.len();        // 行数
        let n = grid[0].len();     // 列数

        // 创建 dp 数组，初始化为 0
        let mut dp = vec![vec![0; n]; m];

        dp[0][0] = grid[0][0]; // 起点

        // 填充第一行
        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }

        // 填充第一列
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }

        // 填充 dp 表
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = grid[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
            }
        }

        dp[m - 1][n - 1] // 返回右下角的最小路径和
    }
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    let result = Solution::min_path_sum(grid); // 正确调用方法
    println!("Minimum path sum: {}", result); // 输出 7
}
