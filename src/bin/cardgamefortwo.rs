use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;

fn main() {
    input! {
        n : u32,
        a : [u32; n],
    }
    let mut a_ = a.clone();
    a_.sort_by(|x, y| y.cmp(x));
    let alice = a_.iter().step_by(2).sum::<u32>();
    let bob = a_.iter().skip(1).step_by(2).sum::<u32>();
    // ### other answer ###
    // alice = a_
    //     .iter()
    //     .enumerate()
    //     .filter(|&(i, _)| i % 2 == 0)
    //     .map(|(_, e)| e)
    //     .sum::<(u32)>();
    // bob = a_
    //     .iter()
    //     .enumerate()
    //     .filter(|&(i, _)| i % 2 != 0)
    //     .map(|(_, e)| e)
    //     .sum::<u32>();
    println!("{}", alice - bob);
}
