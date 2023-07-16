rust
#![feature(box_syntax)]

pub fn foo() -> u32 {
    let a =  box [0_u32; u32::MAX as usize];
    a[0]
}

fn main() {
    println!("{}", foo());
}
