 rust
// This is fine.
const X: u32 = 50;
const Y: *const u32 = &X;
println!("{:?}", Y);
