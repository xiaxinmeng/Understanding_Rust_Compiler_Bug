Rust
struct Bug<F: Fn(&u8) = fn() -> &'static u8> {
    F: P,
}
