rust
use std::process::Termination;

fn foo<T: Termination>(_: T) {}

fn main() {
    foo(1_i32);
}
