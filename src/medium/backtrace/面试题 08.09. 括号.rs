// 面试题 08.09. 括号
struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        Self::dfs(0, 0, n, "".to_string(), &mut result);
        result
    }
    fn dfs(left: i32, right: i32, n: i32, cur: String, result: &mut Vec<String>) {
        if left == n && right == n {
            result.push(cur);
            return;
        }
        if right < left {
            Self::dfs(left, right + 1, n, cur.clone() + ")", result);
        }
        if left < n {
            Self::dfs(left + 1, right, n, cur.clone() + "(", result);
        }
    }
}
fn main() {
    let ans = Solution::generate_parenthesis(4);
    print!("{:?}", ans);
}
