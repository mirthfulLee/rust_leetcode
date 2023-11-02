// 221. 最大正方形

impl Solution {

    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut dp:Vec<Vec<i32>> = vec![vec![0;col]; row];
        let mut result = 0;
        for i in 0..row {
            for j in 0..col {
                if matrix[i][j] == '1' {
                    if j == 0 || i == 0{
                        dp[i][j] = 1;
                    }else {
                        dp[i][j] = dp[i-1][j-1].min(dp[i-1][j].min(dp[i][j-1])) + 1;
                    }
                    result = result.max(dp[i][j]);
                }
            }
        }
        result * result
    }
}