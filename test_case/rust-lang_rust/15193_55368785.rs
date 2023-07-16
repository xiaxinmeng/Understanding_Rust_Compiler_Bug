 rust
fn main() {
    let _: Box<Num+Send> = 1i;
}
// error: mismatched types: expected `Box<core::num::Num+Send>`, found `int` (expected box, found int)
