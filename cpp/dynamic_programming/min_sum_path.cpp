class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
        int row = grid.size(), col = grid[0].size();
        if(row == 0){return 0;}

        int dp[row][col];
        dp[0][0] = grid[0][0];
        for(int c=1; c < col; c++){
            dp[0][c] = grid[0][c] + dp[0][c-1];
        }
        for(int r=1; r < row; r++){
            dp[r][0] = grid[r][0] + dp[r-1][0];
        }
        for(int i=1; i < row; i++){
            for(int j = 1; j<col; j++){
                dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1]);
            }
        }
        return dp[row-1][col-1];
    }
};
