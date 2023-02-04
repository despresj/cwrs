#![allow(dead_code)]

pub fn largest_five_digit_number_possible(num: &str) -> u32 {
    let mut char_array: Vec<char> = vec![];
    for char in num.chars() {
        char_array.push(char);
    }
    char_array.sort_unstable();
    char_array.reverse();
    let mut string_of_char_nums = String::new();
    for number in char_array {
        string_of_char_nums.push(number);
    }

    let sub = String::from(&string_of_char_nums[0..5]);
    let result: u32 = sub.parse().unwrap();

    result
}

// In the following 6 digit number:
//
// 283910
// 91 is the greatest sequence of 2 consecutive digits.
//
// In the following 10 digit number:
//
// 1234567890
// 67890 is the greatest sequence of 5 consecutive digits.
//
// Complete the solution so that it returns the greatest sequence of five consecutive digits found within the number given. The number will be passed in as a string of only digits. It should return a five digit integer. The number passed may be as large as 1000 digits.
//
pub fn largest_five_digit_number(num: &str) -> u32 {
    let mut numeric_array: Vec<u32> = vec![];
    for (n, _) in num.chars().enumerate() {
        let to_slice = num.clone();

        if n + 5 > to_slice.len() {
            break;
        }

        let digits: u32 = to_slice[n..n + 5].parse().unwrap();
        numeric_array.push(digits);
    }
    let output = numeric_array.iter().max().unwrap();
    *output
}

/// Return a String with all characters masked as '#' except the last 4.
// Usually when you buy something, you're asked whether your credit card number, phone number or answer to your most secret question is still correct. However, since someone could look over your shoulder, you don't want that shown on your screen. Instead, we mask it.
//
// Your task is to write a function maskify, which changes all but the last four characters into '#'.
//
// Examples
//
// "4556364607935616" --> "############5616"
//      "64607935616" -->      "#######5616"
//                "1" -->                "1"
//                 "" -->                 ""
//
// // "What was the name of your first pet?"
//
// "Skippy" --> "##ippy"
//
// "Nananananananananananananananana Batman!"
// "####################################man!"

// https://www.codewars.com/kata/5412509bd436bd33920011bc/train/rust
pub fn maskify(cc: &str) -> String {
    let length = cc.len();
    if length < 5 {
        return cc.to_string();
    }

    format!("{}{}", "#".repeat(&length - 4), &cc[length - 4..length]).to_string()
}

// Given an integer as input, can you round it to the next
// (meaning, "greater than or equal") multiple of 5?
//
// Examples:
//
// input:    output:
// 0    ->   0
// 2    ->   5
// 3    ->   5
// 12   ->   15
// 21   ->   25
// 30   ->   30
// -2   ->   0
// -5   ->   -5
// etc.
// Input may be any positive or negative integer (including 0).
//
// You can assume that all inputs are valid integers.
pub fn round_to_next_5(n: i32) -> i32 {
    if n % 5 == 0 {
        return n;
    }

    if n > 0 {
        println!("n = {}, res = {}", n, n - n % 5 + 5);

        return n - (n % 5) + 5;
    } else {
        println!("n = {}, res = {}", n, n - n % 5);
        return n - n % 5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_round() {
        assert_eq!(round_to_next_5(1), 5);
    }

    #[test]
    fn test_basic_neg() {
        assert_eq!(round_to_next_5(-1), 0);
    }

    #[test]
    fn test_extended() {
        let tests = [
            [0, 0],
            [1, 5],
            [-1, 0],
            [-5, -5],
            [3, 5],
            [5, 5],
            [7, 10],
            [20, 20],
            [39, 40],
            [990, 990],
            [121, 125],
            [555, 555],
        ];

        for [x, out] in tests.iter() {
            assert_eq!(round_to_next_5(*x), *out);
        }
    }
}

#[test]
fn it_masks_example_strings() {
    assert_eq!(maskify("4556364607935616"), "############5616");
    assert_eq!(maskify("5616"), "5616");
    assert_eq!(maskify("1"), "1");
    assert_eq!(maskify("11111"), "#1111");
}
#[test]
fn test_basic_cooler() {
    assert_eq!(largest_five_digit_number_possible(&"1234567890"), 98765);
    assert_eq!(largest_five_digit_number_possible(&"48894134"), 98844);
    assert_eq!(largest_five_digit_number_possible(&"1199991991"), 99999);
    assert_eq!(largest_five_digit_number_possible(&"731674765"), 77766);
}

#[test]
fn test_basic() {
    assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
    assert_eq!(largest_five_digit_number(&"731674765"), 74765);
}
