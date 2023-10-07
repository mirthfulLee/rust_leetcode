// 1108. IP 地址无效化
struct Solution;
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut result = String::new();
        for c in address.chars() {
            // result += if c == '.' { "[.]" } else { &c.to_string() }; // temporary value

            let str = &c.to_string();
            result += if c == '.' { "[.]" } else { str };
        }
        result
    }
    pub fn defang_i_paddr2(address: String) -> String {
        let mut result = String::new();
        for c in address.chars() {
            if c == '.' {
                result.push_str("[.]");
            } else {
                result.push(c);
            }
        }
        result
    }
    pub fn defang_i_paddr3(address: String) -> String {
        address.replace(".", "[.]")
    }
    pub fn defang_i_paddr4(address: String) -> String {
        let mut result = String::new();
        for c in address.chars() {
            match c {
                '.' => result.push_str("[.]"),
                _ => result.push(c),
            }
        }
        result
    }
}
fn main() {
    let addr = String::from("255.100.50.0");
    print!("{}", Solution::defang_i_paddr(addr));
}
