// 85. 最大矩形

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut above_ones: Vec<Vec<i32>> = vec![vec![0; col]; row];
        let mut result = 0;
        for i in 0..row {
            for j in 0..col {
                if matrix[i][j] == '1' {
                    if i == 0 {
                        above_ones[i][j] = 1;
                    } else {
                        above_ones[i][j] = above_ones[i - 1][j] + 1;
                    }
                    let mut height = above_ones[i][j];
                    for k in (0..=j).rev() {
                        height = height.min(above_ones[i][k]);
                        if height == 0 {
                            break;
                        }
                        result = result.max(height * (j - k + 1) as i32);
                    }
                }
            }
        }
        result
    }
}
