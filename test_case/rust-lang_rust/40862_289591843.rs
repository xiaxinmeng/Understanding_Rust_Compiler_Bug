rust
fn main() {
    vec![0u8; 1 << 24].into_boxed_slice();
}
