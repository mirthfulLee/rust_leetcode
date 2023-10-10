// 881. 救生艇
struct Solution;
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let mut result = 0;
        let mut l = 0;
        for r in (0..people.len()).rev() {
            if people[l] + people[r] <= limit {
                l += 1;
            }
            result += 1;
            if r <= l {
                break;
            }
        }
        result
    }
}
fn main() {
    let people = vec![1, 2];
    let limit = 5;
    let result = Solution::num_rescue_boats(people, limit);
    print!("{}", result);
}
