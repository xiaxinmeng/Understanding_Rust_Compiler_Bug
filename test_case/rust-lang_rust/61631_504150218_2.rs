rust
struct Bug<F: Fn(&u8) = fn() -> &'static u8> {
    f: F,
}
