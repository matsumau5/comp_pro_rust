pub fn hoge<'a>(a:i32, b:i32) -> i32 {
    a + b
}

pub fn main(){
    println!("{}", hoge(8, 2));
}