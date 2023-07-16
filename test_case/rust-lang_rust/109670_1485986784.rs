rust
fn main() {
    for _ in 0..4 {
        let a = [0u8; 1024 * 1024 * 1024];
        drop(&a[..]);
    }
}
