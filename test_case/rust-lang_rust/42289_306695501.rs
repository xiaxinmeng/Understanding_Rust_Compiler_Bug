rust
const FOO: &'static [u8] = b"foo";

fn test(bytes: &[u8]) {
    match Some(bytes) {
        Some(FOO) => {}
        _ => {}
    }
}

fn main() {
    test(FOO);
}
