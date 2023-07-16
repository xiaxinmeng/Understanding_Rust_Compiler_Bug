 rust
struct S;

impl S {
    pub fn public() -> impl Clone {
        S::private()
    }
    fn private() -> impl Clone {
        0u8
    }
}
