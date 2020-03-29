use comp_pro_rust::input;
use comp_pro_rust::input_inner;
use comp_pro_rust::read_value;
// extern crate regex;

// use regex::Regex;

fn main() {
    input! {
        s : String,
    }
    let patterns = ["eraser", "erase", "dreamer", "dream"];
    let result = patterns.iter().fold(s, |s, x| s.replace(x, " "));
    println!("{}", if result.replace(" ", "").is_empty() { "YES" } else { "NO "});
    // another answer. you need Regex crate.
    // let re = Regex::new(r"^(eraser?|dream(er)?)*$").unwrap();
    // println!(
    //     "{}",
    //     if re.replace_all(&s, "") == "" {
    //         "YES"
    //     } else {
    //         "NO"
    //     }
    // );
}
