 rust
fn use_slice(s: &[u8]) {}
fn main() {
    let s: [u8, ..3] = [0,1,2];
    use_slice(s);
}
