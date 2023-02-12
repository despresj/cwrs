#![allow(dead_code)]
#![allow(unused_variables)]
mod cws;
pub fn get_middle(s: &str) -> &str {
    dbg!(s.len());
    let n_chars = s.len();

    if &n_chars % 2 == 0 {
        let half = n_chars / 2;
        output = &s[half - 1..half + 1];
        return output;
    } else {
        let half = n_chars / 2;
        let output = &s[half..half + 1];
        return output;
    }
}

fn main() {
    println!("result = {:?}", get_middle("strippers"));
}

#[test]
fn get_middle_test() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
}
