#![allow(dead_code)]
#![allow(unused_variables)]
mod cws;

fn main() {
    println!(
        "result = {:?}",
        cws::find_short("this many words is five")
    );
}
