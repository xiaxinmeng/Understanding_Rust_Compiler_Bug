 Rust
use std::mem::align_of;
struct F([u32; 0]);
println!("{:p}", Box::new(F)); //prints 0x1
println!("{}", align_of::<F>()); //prints 4
