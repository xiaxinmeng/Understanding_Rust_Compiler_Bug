rust
struct S {
    #[rustfmt::skip]
    pub(in nonexistent) field: u8 // OK, but should be an error
}

fn main() {}
