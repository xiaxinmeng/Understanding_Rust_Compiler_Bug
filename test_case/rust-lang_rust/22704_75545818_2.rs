 rust
fn main() {
    use foo::Bar;
    let x: Bar<u8>;// Error
    let x: Bar<u8, u8>;
}
