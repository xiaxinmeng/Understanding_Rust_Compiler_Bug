rust
pub struct Foo {
    _x: i32,
}

#[no_mangle]
pub extern "C" fn foo(_: *const Foo) {}
