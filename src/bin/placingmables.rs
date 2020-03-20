use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input! {
        s : String = input!()
    }
    // println!("{}", s.chars().filter(|&c| c == '1').count());
    let ans: Vec<&str> = s.matches("1").collect();
    println!("{}", ans.len());
}
