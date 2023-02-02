fn main() {
    // https://www.codewars.com/kata/51675d17e0c1bed195000001/train/rust
    //
    pub fn largest_five_digit_number(num: &str) -> u32 {
        let mut array: Vec<char> = vec![];
        for char in num.chars() {
            array.push(char);
        }
        array.sort_unstable();
        array.reverse();
        let mut append = String::new();
        for number in array {
            append.push(number);
        }

        let result: u32 = append.parse().unwrap();
        result
    }
    let num = "199991";
    let output = largest_five_digit_number(num);
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
        assert_eq!(largest_five_digit_number(&"731674765"), 74765);
    }
}
