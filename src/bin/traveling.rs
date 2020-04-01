use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input! {
        n : u32,
        val : [(i32, i32, i32); n],
    }
    let mut v: Vec<(i32, i32, i32)> = val[..].to_vec();
    v.insert(0, (0, 0, 0));
    let yes = v[..].windows(2).all(|w| {
        let (t, x, y) = w[0];
        let (nt, nx, ny) = w[1];
        let time = nt - t;
        let dist = (nx - x).abs() + (ny - y).abs();
        dist <= time && time % 2 == dist % 2
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
