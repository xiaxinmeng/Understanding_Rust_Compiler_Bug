 rust
{
    use std::slice::BoxedSlice;
    let xs: ::std::boxed::Box<[_]> = box() [0u, ..16];
    xs.into_vec()
}
