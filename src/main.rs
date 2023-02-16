#![allow(dead_code)]
#![allow(unused_variables)]
mod cws;

fn main() {
    println!("result = {:?}", cws::elevator_distance(&[2, 5, 1]));
}
