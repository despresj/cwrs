#![allow(dead_code)]
#![allow(unused_variables)]
mod cws;

fn main() {
    println!("result = {:?}", cws::find_outlier(&[2, 9, 5, 6]));
}
