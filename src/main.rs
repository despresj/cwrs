struct Solution {
    common: String,
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut common_chars = String::new();
        for c in str1.chars() {
            for (j, k) in str2.chars().enumerate() {
                println!("i = {}, c = {}", i, c);
                println!("j = {}, k = {}", j, k);
                if c == k {
                    common_chars.push(c);
                } else {
                    println!("else");
                }
            }
        }
        common_chars
    }
}

fn main() {
    let str1 = String::from("ABCABC");
    let str2 = String::from("ABC");
    let solution = Solution {
        common: Solution::gcd_of_strings(str1, str2),
    };
    println!("{}", solution.common);
}
