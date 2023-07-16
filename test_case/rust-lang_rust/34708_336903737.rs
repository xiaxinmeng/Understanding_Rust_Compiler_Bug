Rust
#![feature(slice_patterns)]

struct NoisyDrop;
impl Drop for NoisyDrop {
    fn drop(&mut self) { println!("splat!"); }
}

fn main() {
    let [x] = [NoisyDrop];
}
