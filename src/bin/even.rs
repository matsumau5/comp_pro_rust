use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input!{
        a: u32,
        b: u32,
    }
    let ans = if (a * b) % 2 == 0 { "Even" } else { "Odd" };
    println!("{}", ans);
}
