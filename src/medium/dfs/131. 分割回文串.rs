// 131. 分割回文串

struct Solution;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let len = s.len();
        let b = s.as_bytes();
        let mut pali: Vec<Vec<bool>> = vec![vec![false; len]; len];
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut stack: Vec<String> = Vec::new();
        for cur_len in 1..=len {
            for i in 0..=len - cur_len {
                let j = i + cur_len - 1;
                if cur_len <= 3 || (i + 1 < len && pali[i + 1][j - 1]) {
                    pali[i][j] = b[i] == b[j];
                }
            }
        }
        Self::dfs(&s, 0, &pali, &mut stack, &mut result);
        result
    }

    fn dfs(s: &String, idx: usize, pali:&Vec<Vec<bool>>, stack: &mut Vec<String>, result:&mut Vec<Vec<String>>) {
        if idx == s.len() {
            result.push(stack.clone());
            return;
        }
        for end in idx..s.len() {
            if pali[idx][end] {
                stack.push(s[idx..=end].to_string());
                Self::dfs(s, end + 1, pali, stack, result);
                stack.pop();
            }
        }
    }
}

fn main() {
    let sol = Solution::partition(String::from("aab"));
}