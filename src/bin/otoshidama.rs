use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input! {
        n : i32,
        y : i32,
    }
    let (x, y, z) = (0..n + 1)
        .flat_map(move |i| {
            (0..n + 1 - i).flat_map(move |j| {
                (0..n + 1 - i - j)
                    .filter(move |k| 10000 * i + 5000 * j + 1000 * k == y && i + j + k == n)
                    .map(move |k| (i, j, k))
                    .collect::<Vec<(i32, i32, i32)>>()
            })
        })
        .collect::<Vec<(i32, i32, i32)>>()
        .pop()
        .unwrap_or((-1, -1, -1));
    println!("{} {} {}", x, y, z);
}
