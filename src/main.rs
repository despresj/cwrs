#[allow(unused_variables)]
mod cw;

fn main() {
    let x: i32 = 223;
    let y: i32 = 223;
    let z: i32 = 3;

    println!("result = {}, answer = 2 ^ 2", cw::rgb(x, y, z));
}
