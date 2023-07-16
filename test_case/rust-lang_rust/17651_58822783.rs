 rust
fn main() {
    (|| { box *([0u].as_slice()) })();
}
