#![allow(dead_code)]
#![allow(unused_variables)]
mod cws;

fn main() {
    println!(
        "result = {:?}",
        cws::hamming("propersterous", "perporsteros")
    );
}
