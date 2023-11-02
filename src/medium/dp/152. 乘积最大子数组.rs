// 152. 乘积最大子数组

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut last_min = nums[0];
        let mut last_max = nums[0];
        let mut max_val = nums[0];
        for i in 1..nums.len() {
            let new_max = i32::max(nums[i], i32::max(last_max * nums[i], last_min * nums[i]));
            let new_min = i32::min(nums[i], i32::min(last_max * nums[i], last_min * nums[i]));
            if new_max > max_val {max_val = new_max;}
            last_max = new_max;
            last_min = new_min;
        }
        max_val
    }
}