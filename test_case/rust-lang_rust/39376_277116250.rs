Rust
#[repr(packed)]
pub struct Foo(u32);
pub struct Bar(u8, Foo);

#[no_mangle]
pub fn foo(b: &Bar) -> u32 {
    b.1 .0
}
