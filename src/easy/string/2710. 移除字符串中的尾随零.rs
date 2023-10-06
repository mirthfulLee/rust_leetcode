// 2710. 移除字符串中的尾随零
struct Solution {}
impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        num.trim_end_matches(|c| c == '0').to_string()
    }
}


