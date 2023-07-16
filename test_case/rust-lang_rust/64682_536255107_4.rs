rust
pub struct S;

impl S {
    fn _f2(
        /// abc  <-- compiles but it should be rejected!
        _p1: u8
    ) {}
}
