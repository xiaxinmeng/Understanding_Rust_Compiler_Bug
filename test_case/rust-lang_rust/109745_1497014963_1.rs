rust
#[repr(packed(8))]
pub struct Foo {
    a: usize,
    b: [u16; 1],
}

pub fn foo(f: &Foo) {
    bar(f.b.as_ptr());
}

pub fn bar(_ptr: *const u16) {}
