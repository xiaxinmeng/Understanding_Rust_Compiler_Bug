rust
#![feature(box_syntax)]

fn main() {
    let a =  box [0_u32; u32::MAX as usize];
    println!("{}", a[0]);
}
