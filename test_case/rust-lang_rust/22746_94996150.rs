 rust
use std::mem;

#[repr(packed)]
pub enum BytecodeInstruction {
A, B, C
}

fn main() {
    println!("Hello, world! {}", std::mem::size_of::<BytecodeInstruction>())
}
