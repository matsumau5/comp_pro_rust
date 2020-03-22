use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;
use std::collections::HashSet;

fn main() {
    input! {
        n : u32,
        a : [u32; n],
    }
    println!("{}", a.into_iter().collect::<HashSet<u32>>().len());
}
