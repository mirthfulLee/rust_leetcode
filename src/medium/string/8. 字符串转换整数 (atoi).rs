// 8. 字符串转换整数 (atoi)

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let len = s.len();
        let mut started = false;
        let chars = s.as_bytes();
        let i = 0;
        let mut val = -1;
        let mut negtive = false;
        for i in 0..s.len() {
            let c = chars[i];
            if c.is_ascii_digit() {
                let cur = c as i32 - b'0' as i32;
                if !started {
                    started = true;
                    val *= cur;
                    continue;
                }
                let floor = i32::MIN / 10;
                if val < floor || (val == floor && -cur <= i32::MIN % 10) { // 负数的余数也是负数
                    val = i32::MIN;
                } else {
                    val = val * 10 - cur;
                }
            } else if started {  // 需要先判断started, 否则遇到如 "012-345"会出错
                break;
            } else if c == b'-' {
                if i < s.len() - 1 && chars[i + 1].is_ascii_digit() {
                    negtive = true;
                } else {
                    break;
                }
            } else if c == b'+' {
                if !(i < s.len() - 1 && chars[i + 1].is_ascii_digit()) {
                    break;
                }
            } else if c != b' ' {
                break;
            }
        }

        return match started {
            false => 0,
            true => match negtive {
                true => val,
                false => {
                    if val == i32::MIN {
                        -(val + 1)
                    } else {
                        -val
                    }
                }
            },
        };
    }
}
