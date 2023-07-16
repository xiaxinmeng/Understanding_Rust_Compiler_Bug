rust
// repro.rs
use lib::{bar, baz};

fn main() {
    bar::call_me();
    baz::call_me();
}
