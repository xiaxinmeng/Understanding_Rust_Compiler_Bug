rust
mod num;
use num::Number;
use std::str::FromStr;
fn main(){
    let s = Number::from_str("6.7").unwrap();
    println!("{:?}", s);
}
