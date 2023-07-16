 rust
// crate_b.rs
extern crate crate_a;

fn main() {
    unsafe { crate_a::foo(); }
}
