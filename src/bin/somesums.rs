use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input! {
        n : u32,
        a : u32,
        b : u32,
    }
    println!(
        "{}",
        (1..=n)
            .filter(|x| {
                let sum = x.to_string().chars().map(|s| s.to_digit(10).unwrap()).sum();
                a <= sum && sum <= b
            })
            .sum::<u32>()
    );
}
