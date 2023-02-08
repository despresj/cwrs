#[allow(unused_variables)]
mod cw;

use regex::Regex;
fn main() {
    println!("result = {}", cw::get_count_regex("aabbcc"));
}
