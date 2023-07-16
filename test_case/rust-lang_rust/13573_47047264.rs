 rust
#![feature(macro_rules)]

macro_rules! dont_capture {($v:expr) => ({let _x = 13; $v})}

fn main() -> (){
    let _x = 2;
    let y = dont_capture!(_x);
    println!("2 = {}",y);
}
