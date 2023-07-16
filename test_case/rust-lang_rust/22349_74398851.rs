 rust
use std::mem::transmute;

fn main() {
    let lit: f32 = 0.9;
    let prs: f32 = "0.9".parse().unwrap();

    let lit_int: u32 = unsafe{transmute(lit)};
    let prs_int: u32 = unsafe{transmute(prs)};

    println!("{:x}", lit_int);
    println!("{:x}", prs_int);
}
