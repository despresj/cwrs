#![allow(dead_code)]
#![allow(unused_variables)]
mod cws;

fn main() {
    println!(
        "result = {:?}",
        cws::find_missing(&[1, 2, 3, 4, 6, 7, 8, 9])
    );
}
