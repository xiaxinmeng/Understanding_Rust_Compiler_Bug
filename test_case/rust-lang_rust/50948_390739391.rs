rust
use std::fmt::Debug;
use std::mem::*;

fn main() {
    let a: &dyn Debug = &1u32;
    let b: &dyn Debug = &1u64;
    println!("{}", align_of_val(a)); // 4
    println!("{}", align_of_val(b)); // 8
    // println!("{}", align_of::<dyn Debug>());  // ???
}
