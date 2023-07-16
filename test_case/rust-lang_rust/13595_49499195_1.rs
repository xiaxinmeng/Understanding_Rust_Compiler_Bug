 rust
static foo_code: [u8, .. N] = [/*machine code*/];

fn main() { let x = &foo_code; } // *not* let x = foo_code;
