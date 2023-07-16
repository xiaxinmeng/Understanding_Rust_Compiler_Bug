rust
#[repr(C)]
pub struct Bar {
    pub foo: u8,
}

#[repr(C)]
pub struct Foo {
    pub a: Bar,
    pub b: u32,
    pub c: u32,
}

#[no_mangle]
pub extern "C" fn foo(a: Foo) {}
