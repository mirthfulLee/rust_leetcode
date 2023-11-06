// 179. 最大数

struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut num_str = nums
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>();
        let cmp = |a: &String, b: &String| -> Ordering {
            String::cmp(&(a.clone() + &b), &(b.clone() + &a))
        };
        num_str.sort_by(cmp);
        let mut result: String = String::new();
        for s in num_str {
            result.push_str(&s);
        }
        if result.as_bytes()[0] == b'0' {  // for situation that all num is 0;
            result.truncate(1);
        }
        result
    }
}

fn main() {
    let nums = Vec::from([3,30,34,5,9]);
    print!("{}", Solution::largest_number(nums));
}
