// 166. 分数到小数
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut flag = 1;
        let mut numr = numerator as i64;
        let mut deno = denominator as i64;
        if numr < 0 {
            numr *= -1;
            flag *= -1;
        }
        if deno < 0 {
            deno *= -1;
            flag *= -1;
        }
        let mut result = (numr / deno).to_string();
        if flag == -1 && numr != 0 {
            result.insert(0, '-');
        }
        numr = numr % deno;
        if numr != 0 {
            result.push('.');
            numr *= 10;
        }
        let mut digit_loc: HashMap<i64, usize> = HashMap::new();
        while numr != 0 {
            let ch = char::from_u32((numr / deno) as u32 + '0' as u32).unwrap();
            if digit_loc.contains_key(&numr) {
                result.insert(digit_loc[&numr], '(');
                result.push(')');
                break;
            } else {
                // digit_loc[&ch] = result.len();
                digit_loc.insert(numr, result.len());
                result.push(ch);
            }
            numr = numr % deno * 10;
        }

        result
    }
}
