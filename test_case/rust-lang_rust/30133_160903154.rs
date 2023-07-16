 rust
mod foo { extern crate core; }
pub use foo::core;
fn main() {
    use core;
}
