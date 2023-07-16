rust
struct Foo<'b> {
    pub my_fn: for<'a> fn(&'a u32) -> &'b u32,
}

// and something equivalent for enums
