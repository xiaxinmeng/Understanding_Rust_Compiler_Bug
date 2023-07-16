 Rust
#![feature(slice_patterns)]

fn main() {
    let [x] = [Box::new(2)];
}
