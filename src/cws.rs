pub fn hamming(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

#[test]
fn simple_hamming_tests() {
    // Translated from the JavaScript test cases.
    assert_eq!(hamming("", ""), 0);
    assert_eq!(hamming("I like turtles", "I like turkeys"), 3);
    assert_eq!(hamming("Hello World", "Hello World"), 0);
    assert_eq!(hamming("a man a plan a canal", "a man a plan sobanal"), 3);
    assert_eq!(hamming("hamming and cheese", "Hamming and Cheese"), 2);
    assert_eq!(hamming("espresso", "Expresso"), 2);
    assert_eq!(
        hamming("old father, old artificer", "of my soul the uncreated "),
        24
    );
}

pub fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names.join("")),
        2 => format!("{} like this", names.join(" and ")),
        3 => format!("{} and {} like this", names[0..2].join(", "), names[2]),
        x => format!("{} and {} others like this", names[0..2].join(", "), x - 2),
    }
}

#[test]
fn example_tests() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}

pub fn count_duplicates(text: &str) -> u32 {
    use std::collections::HashMap;

    let mut count_map = HashMap::new();
    for letter in text.chars() {
        let count = count_map.entry(letter).or_insert(0);
        *count += 1;
    }

    let mut num_repeated: Vec<i32> = Vec::new();
    for (letter, count) in count_map {
        if count > 1 {
            num_repeated.push(count);
        }
    }

    num_repeated.len().try_into().unwrap()
}

#[test]
fn test_count_duplicates() {
    assert_eq!(count_duplicates("abcde"), 0);
    assert_eq!(count_duplicates("indivisibility"), 1);
    assert_eq!(count_duplicates("abcdea"), 1);
}

pub fn high_and_low(numbers: &str) -> String {
    let iter_nums: Vec<i32> = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let minimum = iter_nums.iter().min().unwrap().to_string();
    let maximum = iter_nums.iter().max().unwrap().to_string();
    [maximum, minimum].join(" ")
}

#[test]
fn test_high_and_low() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    assert_eq!("3 1", high_and_low("1 2 3"));
}

pub fn to_camel_case(text: &str) -> String {
    let mut output: Vec<String> = Vec::new();
    let mut cap_next = false;

    for letter in text.chars() {
        if letter == '_' || letter == '-' {
            cap_next = true;
        } else {
            if cap_next {
                output.push(letter.to_ascii_uppercase().to_string());
                cap_next = false;
            } else {
                output.push(letter.to_string());
                cap_next = false;
            }
        }
    }
    output.join("")
}

const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

fn dotest(s: &str, expected: &str) {
    assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
}

#[test]
fn camel_case_test() {
    dotest("", "");
    dotest("the_stealth_warrior", "theStealthWarrior");
    dotest("The-Stealth-Warrior", "TheStealthWarrior");
    dotest("A-B-C", "ABC");
}

pub fn elevator_distance(floors: &[i16]) -> i16 {
    floors
        .iter()
        .zip(floors.iter().skip(1))
        .map(|(floor, next_floor)| (floor - next_floor).abs())
        .sum()
}

#[test]
fn elevator_distance_test() {
    assert_eq!(elevator_distance(&[5, 2, 8]), 9);
    assert_eq!(elevator_distance(&[1, 2, 3]), 2);
    assert_eq!(elevator_distance(&[7, 1, 7, 1]), 18);
}

pub fn disemvowel(s: &str) -> String {
    s.replace(&['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'], "")
}

#[test]
fn example_test() {
    assert_eq!(
        disemvowel("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
}
pub fn five_div(num: i32) -> i32 {
    (0..num)
        .into_iter()
        .filter(|x| x % 5 == 0 || x % 3 == 0)
        .sum()
}

#[test]
fn test_five_div() {
    // assertion(expected, input);
    assertion(23, 10);
    assertion(33, 11);
    assertion(225, 33);
    assertion(8, 6);
    assertion(3420, 123);
    assertion(543, 50);
    assertion(0, 0);
    assertion(0, -203);
    assertion(25_719_750, 10_500);
}

fn assertion(expected: i32, input: i32) {
    let actual = five_div(input);

    assert!(
        expected == actual,
        "\nTest failed!\n expected: {expected}\n actual: {actual}\n input: {input}\n"
    );
}
pub fn accum(s: &str) -> String {
    let mut output: Vec<String> = Vec::new();
    for (i, char) in s.chars().enumerate() {
        let iter = char.to_string().to_lowercase().repeat(i);
        output.push(char.to_string().to_uppercase() + &iter);
    }

    output.join("-")
}

pub fn spin_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(|x| {
            if x.len() > 4 {
                x.chars().rev().collect::<String>()
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn find_short(s: &str) -> u32 {
    let word_split = s.split_whitespace();
    let lens = word_split.clone().map(str::len).min().unwrap_or(0);
    lens as u32
}

pub fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

pub fn get_middle(s: &str) -> &str {
    let n_chars = s.len();
    if n_chars % 2 == 0 {
        let half = n_chars / 2;
        &s[(half - 1)..=half]
    } else {
        let half = n_chars / 2;
        &s[half..=half]
    }
}

pub fn find_outlier(values: &[i32]) -> i32 {
    let mut even_count = 0;
    let mut odd_count = 0;

    for value in values {
        if value % 2 == 0 {
            even_count += 1
        }
        if value % 2 != 0 {
            odd_count += 1
        }
    }

    if even_count == 1 {
        let output = values.iter().filter(|x| *x % 2 == 0).min();
        println!("a = {:?}", &output.unwrap());
        return *output.expect("minimum value");
    }

    if odd_count == 1 {
        let output = values.iter().filter(|x| *x % 2 != 0).min();
        return *output.expect("min values");
    }

    panic!("We should not have made it past both checks")
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
    let number_vec: Vec<i32> = Vec::from_iter(0..=n);
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
    let set = Regex::new(r"a|e|i|o|u").unwrap();
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

    format!("{}{}", "#".repeat(&length - 4), &cc[length - 4..length])
}

pub fn round_to_next_5(n: i32) -> i32 {
    if n % 5 == 0 {
        return n;
    }

    if n > 0 {
        println!("n = {}, res = {}", n, n - n % 5 + 5);

        n - (n % 5) + 5
    } else {
        println!("n = {}, res = {}", n, n - n % 5);
        n - n % 5
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

        for [x, out] in &tests {
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
    assert_eq!(largest_five_digit_number_possible("1234567890"), 98765);
    assert_eq!(largest_five_digit_number_possible("48894134"), 98844);
    assert_eq!(largest_five_digit_number_possible("1199991991"), 99999);
    assert_eq!(largest_five_digit_number_possible("731674765"), 77766);
}

#[test]
fn test_basic() {
    assert_eq!(largest_five_digit_number("1234567890"), 67890);
    assert_eq!(largest_five_digit_number("731674765"), 74765);
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
        206_847_684,
        1_056_521,
        7,
        17,
        1901,
        21_104_421,
        7,
        1,
        35521,
        1,
        7781,
    ];
    let t3 = [std::i32::MAX, 0, 1];
    assert_eq!(3, find_outlier(&t1));
    assert_eq!(206_847_684, find_outlier(&t2));
    assert_eq!(0, find_outlier(&t3));
}
#[test]
fn get_middle_test() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
}
#[test]
fn repeat_str_test() {
    assert_eq!(repeat_str("a", 4), "aaaa");
    assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(repeat_str("abc", 2), "abcabc");
}

fn dotest_find_short_test(s: &str, expected: u32) {
    let actual = find_short(s);
    assert!(
        actual == expected,
        "With s = \"{s}\"\nExpected {expected} but got {actual}"
    )
}

#[test]
fn find_shortest_tests() {
    dotest_find_short_test("bitcoin take over the world maybe who knows perhaps", 3);
    dotest_find_short_test(
        "turns out random test cases are easier than writing out basic ones",
        3,
    );
    dotest_find_short_test("lets talk about javascript the best language", 3);
    dotest_find_short_test("i want to travel the world writing code one day", 1);
    dotest_find_short_test("Lets all go on holiday somewhere very cold", 2);
    dotest_find_short_test("Let's travel abroad shall we", 2);
}

#[test]
fn spin_words_test() {
    assert_eq!(spin_words("Welcome"), "emocleW");
    assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(spin_words("This is a test"), "This is a test");
    assert_eq!(spin_words("This is another test"), "This is rehtona test");
    assert_eq!(
        spin_words("You are almost to the last test"),
        "You are tsomla to the last test"
    );
    assert_eq!(
        spin_words("Just kidding there is still one more"),
        "Just gniddik ereht is llits one more"
    );
    assert_eq!(
        spin_words("Seriously this is the last one"),
        "ylsuoireS this is the last one"
    );
}
#[test]
fn basic_testing_accum() {
    assert_eq!(
        accum("ZpglnRxqenU"),
        "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("NyffsGeyylB"),
        "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
    );
    assert_eq!(
        accum("MjtkuBovqrU"),
        "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("EvidjUnokmM"),
        "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
    );
    assert_eq!(
        accum("HbideVbxncC"),
        "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
    );
    assert_eq!(accum("ijk"), "I-Jj-Kkk");
    assert_eq!(accum("ztk"), "Z-Tt-Kkk");
}
