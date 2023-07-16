 rust
#![crate_type="lib"]
mod b {
    mod a {
        pub struct S1;
        pub struct S2(pub S1);
    }
    pub use self::a::S2;
}

pub fn g(s : b::S2) {
    let s = s.0; // Oops, I can't name the type of the field.
}
