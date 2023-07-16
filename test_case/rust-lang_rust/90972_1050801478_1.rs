Rust
bash-5.1# cat neon.rs 
use std::arch::aarch64::*;

pub fn main() {
    let a = unsafe { vdupq_n_u8(7) };
    println!("a = {:?}", a);
}

bash-5.1# rustc neon.rs 

bash-5.1# ./neon 
a = uint8x16_t(7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7)

bash-5.1# rustc --version
rustc 1.59.0 (9d1b2106e 2022-02-23)
