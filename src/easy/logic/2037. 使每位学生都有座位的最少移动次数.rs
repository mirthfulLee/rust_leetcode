// 2037. 使每位学生都有座位的最少移动次数
struct Solution;
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut sorted_sts = seats.clone();
        let mut sorted_stu = students.clone();
        sorted_sts.sort();
        sorted_stu.sort();
        let mut sum = 0;
        for i in 0..seats.len() {
            sum += (sorted_sts[i] - sorted_stu[i]).abs();
        }
        sum
    }

    pub fn min_moves_to_seat_mut_version(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort();
        students.sort();
        let mut sum = 0;
        for i in 0..seats.len() {
            sum += (seats[i] - students[i]).abs();
        }
        sum
    }
}
fn main() {
    let seats = vec![4, 1, 5, 9];
    let students = vec![1, 3, 2, 6];
    print!("{}", Solution::min_moves_to_seat(seats, students));
}
