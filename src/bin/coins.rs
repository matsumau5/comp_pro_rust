use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input! {
        a : u32,
        b : u32,
        c : u32,
        x : u32,
    }
    println!(
        "{}",
        (0..=a)
            .flat_map(|i| {
                (0..=b).flat_map(move |j| {
                    (0..=c)
                        .map(|k| 500 * i + 100 * j + 50 * k)
                        .collect::<Vec<u32>>()
                })
            })
            .filter(|&n| n == x)
            .count()
    );
}
