rust
extern "C" {
    static foo: Weak<fn(usize) -> *const u8>;
}
