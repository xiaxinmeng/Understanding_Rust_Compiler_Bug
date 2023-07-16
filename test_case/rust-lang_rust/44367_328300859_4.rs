rust
static mut FOO: fn(u8x32<CRATE_ABI>) = default; // OK, compiles
fn default(a: u8x32<CRATE_ABI>) { }
