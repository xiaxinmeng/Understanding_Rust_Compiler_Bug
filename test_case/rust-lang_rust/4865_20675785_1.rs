 rust
// bar.rs
extern mod foo;
use foo::*;

fn main() {
    foo();
}
