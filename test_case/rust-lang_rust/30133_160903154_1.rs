 rust
mod foo { extern crate core; }
pub use foo::*;
fn main() {
    use core;
}
