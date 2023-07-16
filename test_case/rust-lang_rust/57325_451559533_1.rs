rust
impl<'a> Foo for () {
    type Bar = &'a u32; // error
}
