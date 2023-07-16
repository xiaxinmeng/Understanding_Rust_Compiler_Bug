rust
#![allow(const_err)]

const VOID: u32 = 0 - 1;

fn main() {
    let x = VOID; // still an error
}
