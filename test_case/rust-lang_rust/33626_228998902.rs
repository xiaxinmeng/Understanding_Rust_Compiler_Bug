 rust
#[repr(align="8")]
struct F;

println!("{:p}", Box::new(F));
// prints 0x1? 0x8?
