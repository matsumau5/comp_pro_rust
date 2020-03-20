use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;
// use std::io::*;

fn main() {
    input! {
        n : u32,
        v : [i64; n],
    }
    //let ans: Vec<i64> = v.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("{}", v.iter().min().unwrap().trailing_zeros());
}
