Rust
fn main() {
    [(); &(static || {}) as *const _ as usize]
}
