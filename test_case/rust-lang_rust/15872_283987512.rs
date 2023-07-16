rust
impl Foo for &str { .. }
impl Foo for Ref<u8> { .. } // Ref is defined as `struct Ref<'a, T>`
