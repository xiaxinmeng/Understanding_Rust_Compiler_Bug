`rust
struct S0<T>(T);
impl<T> S0<T> {
    fn foo() {
        Self(0);
        const C: S0<u8> = Self(0);
    }
}
