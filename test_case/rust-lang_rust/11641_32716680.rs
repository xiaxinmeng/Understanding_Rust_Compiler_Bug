 rust
static FOO: &'static [u8] = {
    static F: &'static [u8] = &'static [10u8];
    F
};
