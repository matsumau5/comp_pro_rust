use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input! {
        n : u32,
        v : [u32; n],
    }
    //let ans: Vec<i64> = v.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("{}", v.iter().map(|u| u.trailing_zeros()).min());
}
