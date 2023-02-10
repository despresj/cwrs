pub fn find_outlier(values: &[i32]) -> i32 {
    let mut count: i32 = 0;

    let odd_count = values.into_iter().map(|x| {
        if x % 2 != 0 {
            count += 1
        }
    });

    dbg!(&count);
    count
}

pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut output = a;
    output.retain(|x| !b.contains(x));
    output
}

pub fn positive_sum(slice: &[i32]) -> i32 {
    let mut output: i32 = 0;
    for digit in slice {
        if digit > &0 {
            output += digit;
        }
    }
    output
}

pub fn summation(n: i32) -> i32 {
    let number_vec: Vec<i32> = Vec::from_iter(0..n + 1);
    number_vec.iter().sum()
}

pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut status: Vec<String> = Vec::new();
    for datum in data {
        let category = match datum {
            x if x.0 >= 55 && x.1 > 7 => "Senior",
            _ => "Open",
        };
        status.push(category.to_string());
    }
    status
}

pub fn get_count(input: &str) -> usize {
    let mut vowels_count: usize = 0;
    for letter in input.chars() {
        if letter == 'a' {
            vowels_count += 1
        } else if letter == 'e' {
            vowels_count += 1
        } else if letter == 'i' {
            vowels_count += 1
        } else if letter == 'o' {
            vowels_count += 1
        } else if letter == 'u' {
            vowels_count += 1
        }
    }
    vowels_count
}

pub fn get_count_regex(input: &str) -> usize {
    use regex::Regex;
    let set = Regex::new(&r"a|e|i|o|u").unwrap();
    set.find_iter(input).count()
}

pub fn number_to_string(i: i32) -> String {
    i.to_string()
}

pub fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

pub fn dna_strand(dna: &str) -> String {
    let mut char_vec: Vec<char> = vec![];
    for letter in dna.chars() {
        let mapping = match letter {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => letter,
        };
        char_vec.push(mapping);
    }
    char_vec.iter().collect()
}

pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut nums: Vec<i64> = Vec::new();
    for (key, answer) in arr_a.iter().zip(arr_b.iter()) {
        let score = match answer {
            ans if ans == key => 4,
            ans if ans.is_empty() => 0,
            _ => -1,
        };
        nums.push(score)
    }
    let sum_score: i64 = nums.iter().sum();
    sum_score.max(0)
}

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

pub fn maskify(cc: &str) -> String {
    let length = cc.len();
    if length < 5 {
        return cc.to_string();
    }

    format!("{}{}", "#".repeat(&length - 4), &cc[length - 4..length]).to_string()
}

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
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
    }

    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(
            actual == expected,
            "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("AAAA", "TTTT");
        dotest("ATTGC", "TAACG");
        dotest("GTAT", "CATA");
    }

    #[test]
    fn test_basic_check() {
        assert_eq!(check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]), 6);
        assert_eq!(check_exam(&["a", "a", "c", "b"], &["a", "a", "b", ""]), 7);
        assert_eq!(check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]), 16);
        assert_eq!(check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]), 0);
    }

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
#[test]
fn fixed_tests() {
    fn do_num_to_str_test(n: i32, expected: &str) {
        let actual = number_to_string(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }
    do_num_to_str_test(67, "67");
    do_num_to_str_test(79585, "79585");
    do_num_to_str_test(1 + 2, "3");
    do_num_to_str_test(1 - 2, "-1");
}
#[test]
fn test_get_count() {
    assert_eq!(get_count("abracadabra"), 5);
}
#[test]
fn test_get_count_regx() {
    assert_eq!(get_count_regex("abracadabra"), 5);
    assert_eq!(get_count_regex("pear tree"), 4);
    assert_eq!(get_count_regex("o a kak ushakov lil vo kashu kakao"), 13);
}
#[test]
fn returns_expected() {
    assert_eq!(
        open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
        vec!["Open", "Senior", "Open", "Senior"]
    );
    assert_eq!(
        open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
        vec!["Open", "Open", "Open", "Open"]
    );
}
#[test]
fn basic_tests() {
    assert_eq!(summation(1), 1);
    assert_eq!(summation(8), 36);
    assert_eq!(summation(22), 253);
    assert_eq!(summation(100), 5050);
    assert_eq!(summation(213), 22791);
}
#[test]
fn some_examples() {
    assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
    assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
    assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
}

#[test]
fn empty_list() {
    assert_eq!(positive_sum(&[]), 0);
}

#[test]
fn all_negative() {
    assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
}
#[test]
fn returns_expected_array_diff() {
    assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
    assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    assert_eq!(array_diff(vec![1, 2, 3], vec![1, 2]), vec![3]);
}
#[test]
fn basic_test_outiars() {
    let t1 = [2, 6, 8, -10, 3];
    let t2 = [
        206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
    ];
    let t3 = [std::i32::MAX, 0, 1];
    assert_eq!(3, find_outlier(&t1));
    assert_eq!(206847684, find_outlier(&t2));
    assert_eq!(0, find_outlier(&t3));
}
