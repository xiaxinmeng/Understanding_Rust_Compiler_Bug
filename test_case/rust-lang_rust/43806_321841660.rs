rust
#![deny(unused_results)]
fn g() {}
fn main() {
    g(); //~ ERROR unused result
}
