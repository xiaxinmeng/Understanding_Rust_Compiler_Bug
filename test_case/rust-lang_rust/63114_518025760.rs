rust
mod m {
    pub struct Z;
    impl Z {
        pub fn public() {}
    }
}
pub struct S {
    field: m::Z,
}
