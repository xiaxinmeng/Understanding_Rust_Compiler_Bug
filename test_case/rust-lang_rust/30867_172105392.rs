 Rust
impl<T> Foo for Option<T> where
    T: Fn(&u32),
    for<'a> <T as Fn(&'a u32)>::Output: Sized
{}

// evaluate <Option<_> as Foo>
