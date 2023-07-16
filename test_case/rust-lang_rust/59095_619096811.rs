rust
pub struct Foo(usize, str);
pub fn foo(v: &Foo) -> &u8 {
    unsafe { &v.1.as_bytes().get_unchecked(1) }
}
