// 2367. 算术三元组的数目
struct Solution;
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut j = 1;
        let mut k = 2;
        let mut cnt = 0;
        let len = nums.len();
        'loopi: for i in 0..len {
            while j < len && nums[j] < nums[i] + diff {
                j += 1;
            }
            if j < len && nums[j] == nums[i] + diff {
                while k < len && nums[k] < nums[j] + diff {
                    k += 1;
                }
                if k < len && nums[k] == nums[j] + diff {
                    cnt += 1;
                    j += 1;
                    k += 1;
                }
            }
            if k >= len {
                break 'loopi;
            }
        }
        cnt
    }
}

fn main() {
    let nums: Vec<i32> = vec![4, 5, 6, 7, 8, 9];
    let diff = 2;
    let result = Solution::arithmetic_triplets(nums, diff);
    print!("{}", result);
}
