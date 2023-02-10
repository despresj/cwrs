#![allow(dead_code)]
#![allow(unused_variables)]
mod cw;

fn main() {
    println!("result = {:?}", cw::find_outlier(&[3, 9, 5, 6]));
}
